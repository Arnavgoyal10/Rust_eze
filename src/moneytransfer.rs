use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;
use crate::models::{Account, SubAccount, Transaction, NewTransaction};
use reqwest::blocking::Client;
use serde::Deserialize;


#[derive(Deserialize)]
struct ExchangeRateResponse {
    conversion_result: f64,
}

pub fn transfer_money(
    conn: &mut PgConnection,
    from_account: Uuid,
    to_account: Uuid,
    amount_to_transfer: f64,
    currency_to_transfer: &str
) -> Result<Transaction, diesel::result::Error> {
    use crate::schema::sub_accounts::dsl::*;
    use crate::schema::transactions::dsl::*;
    
    // Check if both accounts have sub-accounts with matching currency
    let from_sub = match sub_accounts
        .filter(account_id.eq(from_account))
        .filter(currency.eq(currency_to_transfer))
        .first::<SubAccount>(conn)
    {
        Ok(account) => account,
        Err(diesel::result::Error::NotFound) => {
            return Err(diesel::result::Error::RollbackTransaction);
        }
        Err(e) => return Err(e),
    };

    let to_sub = match sub_accounts
        .filter(account_id.eq(to_account))
        .filter(currency.eq(currency_to_transfer))
        .first::<SubAccount>(conn)
    {
        Ok(account) => account,
        Err(diesel::result::Error::NotFound) => {
            return Err(diesel::result::Error::RollbackTransaction);
        }
        Err(e) => return Err(e),
    };
        
    // Verify sufficient balance
    if from_sub.balance < amount_to_transfer {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    // Update balances
    update_balance(conn, from_sub.id, -amount_to_transfer)?;
    update_balance(conn, to_sub.id, amount_to_transfer)?;

    // Create the transaction record
    let new_transaction = NewTransaction {
        sub_account_id_from: Some(from_sub.id),
        sub_account_id_to: Some(to_sub.id),
        amount: amount_to_transfer,
        transfer_currency: currency_to_transfer,
    };

    diesel::insert_into(transactions)
        .values(&new_transaction)
        .returning(Transaction::as_returning())
        .get_result(conn)
}

pub fn update_balance(
    conn: &mut PgConnection,
    sub_account_id: Uuid,
    amount_change: f64
) -> Result<SubAccount, diesel::result::Error> {
    use crate::schema::sub_accounts::dsl::*;
    
    diesel::update(sub_accounts.find(sub_account_id))
        .set(balance.eq(balance + amount_change))
        .returning(SubAccount::as_returning())
        .get_result(conn)
}

pub fn get_balance(
    conn: &mut PgConnection,
    account_id_to_get_balance: Uuid,
    currency_to_get_balance: &str
) -> Result<f64, diesel::result::Error> {
    use crate::schema::sub_accounts::dsl::*;
    let sub_account = sub_accounts
        .filter(account_id.eq(account_id_to_get_balance))
        .filter(currency.eq(currency_to_get_balance))
        .first::<SubAccount>(conn)?;
    Ok(sub_account.balance)
}

pub fn transfer_between_sub_accounts(
    conn: &mut PgConnection,
    account_id_to_use: Uuid,
    from_currency: &str,
    to_currency: &str,
    amount_to_transfer: f64
) -> Result<Transaction, diesel::result::Error> {
    use crate::schema::sub_accounts::dsl::*;
    use crate::schema::transactions::dsl::*;

    // Get the source and destination sub-accounts
    let from_sub = sub_accounts
        .filter(account_id.eq(account_id_to_use))
        .filter(currency.eq(from_currency))
        .first::<SubAccount>(conn)?;

    let to_sub = sub_accounts
        .filter(account_id.eq(account_id_to_use))
        .filter(currency.eq(to_currency))
        .first::<SubAccount>(conn)?;

    // Verify sufficient balance
    if from_sub.balance < amount_to_transfer {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    // Fetch live conversion rate
    let client = Client::new();
    let url = format!("https://v6.exchangerate-api.com/v6/dd4a4841dba843af350365ac/pair/{}/{}/{}", from_currency, to_currency, amount_to_transfer);
    let response = client.get(&url).send().map_err(|_| diesel::result::Error::RollbackTransaction)?;

    // Check if the response is successful
    if !response.status().is_success() {
        println!("Error: {:?}", response.status());
        return Err(diesel::result::Error::RollbackTransaction);
    }

    // Deserialize the JSON response
    let exchange_rate: ExchangeRateResponse = response.json().map_err(|_| diesel::result::Error::RollbackTransaction)?;

    // Use the conversion_result
    let converted_amount = exchange_rate.conversion_result;

    // Update balances
    update_balance(conn, from_sub.id, -amount_to_transfer)?;
    update_balance(conn, to_sub.id, converted_amount)?;

    // Create the transaction record
    let new_transaction = NewTransaction {
        sub_account_id_from: Some(from_sub.id),
        sub_account_id_to: Some(to_sub.id),
        amount: amount_to_transfer,
        transfer_currency: from_currency,
    };

    diesel::insert_into(transactions)
        .values(&new_transaction)
        .returning(Transaction::as_returning())
        .get_result(conn)
}


pub fn get_transactions(
    conn: &mut PgConnection,
    account_id_temp: Uuid
) -> Result<Vec<Transaction>, diesel::result::Error> {
    use crate::schema::sub_accounts::dsl::*;
    use crate::schema::transactions::dsl::*;
    
    // we need to get all transactions for all sub-accounts associated with the account
    // first we get all sub-accounts for the account
    let sub_accounts_temp = sub_accounts
        .filter(account_id.eq(&account_id_temp))
        .load::<SubAccount>(conn)?;
    
    // then we get all transactions for each sub-account
    let sub_account_ids: Vec<_> = sub_accounts_temp.iter().map(|sa| sa.id).collect();
    let transactions_temp = transactions
        .filter(sub_account_id_from.eq_any(&sub_account_ids))
        .or_filter(sub_account_id_to.eq_any(&sub_account_ids))
        .load::<Transaction>(conn)?;

    Ok(transactions_temp)
}
use diesel::{prelude::*, update};
use diesel::pg::PgConnection;
use uuid::Uuid;
use bigdecimal::BigDecimal;
// use chrono::Utc;
use dotenvy::dotenv;
use std::env;
use crate::models::{Account, SubAccount, Transaction, NewTransaction};
// use crate::schema::accounts;




pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_account(conn: &mut PgConnection, holder_name: &str) -> Result<Account, diesel::result::Error> {
    if check_duplicate_account(conn, holder_name) {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    use crate::schema::accounts::dsl::*;
    let new_account = crate::models::NewAccount {
        account_holder_name: holder_name,
        status: "active",
    };

    Ok(diesel::insert_into(accounts)
        .values(&new_account)
        .returning(Account::as_returning())
        .get_result(conn)
        .expect("Error saving new account"))
}

pub fn create_sub_account(conn: &mut PgConnection, account_id_temp: Uuid, currency_temp: &str, balance_temp: f64) -> Result<SubAccount, diesel::result::Error> {
    use crate::schema::sub_accounts::dsl::*;
    if check_duplicate_sub_account(conn, account_id_temp, currency_temp) {
        return Err(diesel::result::Error::RollbackTransaction);
    }
    let new_sub_account = crate::models::NewSubAccount {
        account_id: Some(account_id_temp),
        currency: currency_temp,
        balance:  balance_temp,
    };

    Ok(diesel::insert_into(sub_accounts)
        .values(&new_sub_account)
        .returning(SubAccount::as_returning())
        .get_result(conn)
        .expect("Error saving new sub account"))
}

pub fn check_duplicate_sub_account(conn: &mut PgConnection, account_id_temp: Uuid, currency_temp: &str) -> bool {
    use crate::schema::sub_accounts::dsl::*;
    let result = sub_accounts
        .filter(account_id.eq(account_id_temp))
        .filter(currency.eq(currency_temp))
        .load::<SubAccount>(conn)
        .expect("Error loading sub account");

    if result.len() > 0 {
        return true;
    } else {
        return false;
    }
}

pub fn check_duplicate_account(conn: &mut PgConnection, holder_name: &str) -> bool {
    use crate::schema::accounts::dsl::*;
    let result = accounts
        .filter(account_holder_name.eq(holder_name))
        .load::<Account>(conn)
        .expect("Error loading account");

    if result.len() > 0 {
        return true;
    } else {
        return false;
    }
}

// to create a new transaction we want to first check if the account_id_from and account_id_to are valid and have valid subaccounts
// of the given currency, if they do then we check if the amount to be transfered is actally in the account from which it is being sent,
// if yes, then we call a function called update_balance
pub fn commit_transaction(
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
        account_id_from: Some(from_sub.id),
        account_id_to: Some(to_sub.id),
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
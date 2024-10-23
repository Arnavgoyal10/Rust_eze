use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;
use bigdecimal::BigDecimal;
// use chrono::Utc;
use dotenvy::dotenv;
use std::env;
use crate::{models::{Account, SubAccount, Record}, schema::sub_accounts::{balance, currency}};
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

pub fn validate_transaction(conn: &mut PgConnection, account_id_from: Uuid, account_id_to: Uuid, amount: f64) -> bool {
    use crate::schema::sub_accounts::dsl::*;
    let result_from = sub_accounts
        .filter(account_id.eq(account_id_from))
        .load::<SubAccount>(conn)
        .expect("Error loading sub account");

    let result_to = sub_accounts
        .filter(account_id.eq(account_id_to))
        .load::<SubAccount>(conn)
        .expect("Error loading sub account");

    if result_from.len() == 0 || result_to.len() == 0 {
        return false;
    }

    let currecy_from = result_from[0].currency.clone();
    let currecy_to = result_to[0].currency.clone();
    let balance_from = result_from[0].balance;
    let balance_to = result_to[0].balance;

    if currecy_from != currecy_to {
        return false;
    }

    if balance_from < amount {
        return false;
    }

    return true;
}

pub fn transfer_amount(conn: &mut PgConnection, account_id_from: Uuid, account_id_to: Uuid, amount: f64, currency_to_transfer: String) -> Result<(), diesel::result::Error> {
    use crate::schema::sub_accounts::dsl::*;
    let result_from = sub_accounts
        .filter(account_id.eq(account_id_from))
        .load::<SubAccount>(conn)
        .expect("Error loading sub account");

    let result_to = sub_accounts
        .filter(account_id.eq(account_id_to))
        .load::<SubAccount>(conn)
        .expect("Error loading sub account");

    let balance_from = result_from[0].balance;
    let balance_to = result_to[0].balance;

    let new_balance_from = balance_from - amount;
    let new_balance_to = balance_to + amount;

    let update_from = diesel::update(sub_accounts.find(account_id_from))
        .set(balance.eq(new_balance_from))
        .execute(conn);

    let update_to = diesel::update(sub_accounts.find(account_id_to))
        .set(balance.eq(new_balance_to))
        .execute(conn);

    match update_from {
        Ok(_) => {
            match update_to {
                Ok(_) => {
                    return Ok(());
                },
                Err(e) => {
                    return Err(e);
                }
            }
        },
        Err(e) => {
            return Err(e);
        }

        create_transaction_entry(conn, account_id_from, account_id_to, amount, currency_to_transfer);
    }
}

pub fn create_transaction_entry(conn: &mut PgConnection, account_id_from: Uuid, account_id_to: Uuid, amount_to_transfer: f64, currency_to_transfer: String) -> Result<(), diesel::result::Error> {
    validate_transaction(conn, account_id_from, account_id_to, amount_to_transfer);
    use crate::schema::records::dsl::*;
    let new_record = crate::models::Record {
        transaction_id: Uuid::new_v4(),
        account_id_from: Some(account_id_from),
        account_id_to: Some(account_id_to),
        created_at: chrono::Utc::now().naive_utc(),
        amount: amount_to_transfer,
        currency: currency_to_transfer,
    };

    Ok(diesel::insert_into(records)
        .values(&new_record)
        .execute(conn)
        .expect("Error saving new record"))
}

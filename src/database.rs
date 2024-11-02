use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;
use crate::models::{Account, SubAccount, Transaction, NewTransaction};





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



use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;
use crate::models::{Account, SubAccount, UsernamePassword, ScheduledTransaction};
use crate::otp_implement::{generate_totp_secret, verify_totp_flow};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;

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


pub fn get_accounts(conn: &mut PgConnection) -> Result<Vec<Account>, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;
    accounts.load::<Account>(conn)
}



pub fn validate_username_password(conn: &mut PgConnection, username_to_validate: &str, password_to_validate: &str) -> Option<Uuid> {
    use crate::schema::username_password::dsl::*;
    
    // First, get the user record by username only
    let result = username_password
        .filter(username.eq(username_to_validate))
        .load::<UsernamePassword>(conn)
        .expect("Error loading username password");

    if let Some(user) = result.get(0) {
        // Verify the password directly against stored hash
        if verify(password_to_validate, &user.passwd)
            .unwrap_or(false) 
        {
            // Only verify TOTP if password is correct
            if verify_totp_flow(conn, username_to_validate).unwrap() {
                return Some(user.account_id.unwrap());
            }
        }
    }
    None
}

pub fn add_username_password(conn: &mut PgConnection, username_to_add: &str, password_to_add: &str, account_id_to_add: Uuid) -> Result<UsernamePassword, diesel::result::Error> {
    use crate::schema::username_password::dsl::*;
    
    // Hash the password
    let hashed_password = hash(password_to_add.as_bytes(), DEFAULT_COST)
        .map_err(|_| diesel::result::Error::RollbackTransaction)?;
    
    let totp_secret_to_add = generate_totp_secret().ok();
    if totp_secret_to_add.is_none() {
        return Err(diesel::result::Error::RollbackTransaction);
    }
    let new_username_password = crate::models::NewUsernamePassword {
        username: username_to_add,
        passwd: &hashed_password,  // Use the hashed password
        totp_secret: totp_secret_to_add.as_deref(),
        account_id: Some(account_id_to_add),
    };

    println!("totp_secret_key: {:?}", totp_secret_to_add);
    

    // Check if username already exists
    let existing_username = username_password
    .filter(username.eq(username_to_add))
    .load::<UsernamePassword>(conn)
    .expect("Error checking username");
    
    if !existing_username.is_empty() {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    Ok(diesel::insert_into(username_password)
        .values(&new_username_password)
        .returning(UsernamePassword::as_returning())
        .get_result(conn)
        .expect("Error saving new username password"))
}

pub fn add_scheduled_transaction(conn: &mut PgConnection, from_account_id_temp: Uuid, to_account_id_temp: Uuid, amount_temp: f64, currency_temp: &str, scheduled_date_temp: NaiveDateTime) -> Result<ScheduledTransaction, diesel::result::Error> {
    use crate::schema::scheduled_transactions::dsl::*;
    let date_to_set = scheduled_date_temp.date();
    let date_to_set_with_time = date_to_set.and_hms_opt(0, 0, 0).unwrap();
    let new_scheduled_transaction_temp = crate::models::NewScheduledTransaction {
        from_account_id: from_account_id_temp,
        to_account_id: to_account_id_temp,
        amount: amount_temp,
        currency: currency_temp,
        scheduled_date: date_to_set_with_time,
        executed: false,
    };

    Ok(diesel::insert_into(scheduled_transactions)
        .values(&new_scheduled_transaction_temp)
        .returning(ScheduledTransaction::as_returning())
        .get_result(conn)
        .expect("Error saving new scheduled transaction"))
}

pub fn get_scheduled_transactions(conn: &mut PgConnection) -> Result<Vec<ScheduledTransaction>, diesel::result::Error> {
    use crate::schema::scheduled_transactions::dsl::*;
    scheduled_transactions.load::<ScheduledTransaction>(conn)
}

pub fn view_scheduled_transactions(conn: &mut PgConnection, account_id_to_view: Uuid) -> Result<Vec<ScheduledTransaction>, diesel::result::Error> {
    use crate::schema::scheduled_transactions::dsl::*;
    scheduled_transactions
        .filter(from_account_id.eq(account_id_to_view))
        .load::<ScheduledTransaction>(conn)
}

pub fn delete_scheduled_transaction(conn: &mut PgConnection, transaction_id_to_delete: Uuid) -> Result<usize, diesel::result::Error> {
    use crate::schema::scheduled_transactions::dsl::*;
    diesel::delete(scheduled_transactions.find(transaction_id_to_delete))
        .execute(conn)
}
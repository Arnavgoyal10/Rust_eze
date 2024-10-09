mod models;
pub mod schema;
mod database;

// use diesel::pg::PgConnection;
use dotenvy::dotenv;
use schema::sub_accounts::account_id;
// use std::env;
use crate::models::Account;
use crate::database::{establish_connection, create_account, create_sub_account};
use diesel::prelude::*;
use crate::models::{SubAccount, NewSubAccount};
use crate::schema::sub_accounts;


fn main() {
    dotenv().ok();  // Load environment variables from .env
    let mut conn = establish_connection();

    // Create or get an account (if "John Doe" exists, it will return the existing one)
    let account_id_temp2 = create_account(&mut conn, "John Doe");
    let account_id_temp = account_id_temp2.unwrap();

    // // Create or get a sub-account with the specified currency for this account
    let sub_account_id_usd = create_sub_account(&mut conn, account_id_temp.id, "USD", 1000.0);

    // // Trying to create the same sub-account with USD again won't create a duplicate
    let sub_account_id_usd_duplicate = create_sub_account(&mut conn, account_id_temp.id, "USD", 1000.0);

    // // Create another sub-account for the same account with a different currency
    let sub_account_id_eur = create_sub_account(&mut conn, account_id_temp.id, "EUR", 500.0);

    let account_id_duplicate = create_account(&mut conn, "John Doe");

    
    println!("Account: {:#?}", account_id);
    println!("Sub-Account (USD) ID: {:#?}", sub_account_id_usd);
    println!("Duplicate Sub-Account (USD) ID: {:#?}", sub_account_id_usd_duplicate);
    println!("Sub-Account (EUR) ID: {:#?}", sub_account_id_eur);
    println!("Account: {:#?}", account_id_duplicate);
}


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
use clap::{Parser, Subcommand};
use regex::Regex;
use std::io::{self, Write};
use uuid::Uuid;
use crate::database::transfer_amount;

#[derive(Parser)]
#[command(name = "Account Manager")]
#[command(about = "A CLI to create accounts and sub-accounts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)] // Derive Subcommand for Commands enum
enum Commands {
    /// Create a new account
    CreateAccount {
        /// Name of the account to create
        #[arg(short, long)]
        name: String,
    },
    /// Create a new sub-account
    CreateSubAccount {
        /// Account ID for the sub-account
        #[arg(short, long)]
        subaccount_insert_account_id: Uuid,
        /// Currency for the sub-account
        #[arg(short, long)]
        currency: String,
        /// Initial balance for the sub-account
        #[arg(short, long)]
        balance: f64,
    },
}

fn validate_account_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    re.is_match(name)
}

fn validate_currency(currency: &str) -> bool {
    let re = Regex::new(r"^[A-Z]{3}$").unwrap();
    re.is_match(currency)
}

fn validate_amount(amount: f64) -> bool {
    amount > 0.0
}

fn create_account_flow(conn: &mut diesel::PgConnection) {
    // Account creation flow
    let mut account_name = String::new();
    print!("Enter account name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut account_name).unwrap();
    let account_name = account_name.trim();

    if validate_account_name(account_name) {
        // Here you would call your account creation logic
        println!("Creating account: {}", account_name);
        // Placeholder: replace with actual database call
        let account = create_account(conn, account_name).expect("Failed to create account");
        println!("Account created: {:#?}", account);

        // Call the Python script to generate TOTP secret
        let account_id = account.id; // Assuming `account` has an `id` field
        let output = std::process::Command::new("python")
            .arg("otp.py")
            .arg("generate")
            .arg(account_id.to_string())
            .output()
            .expect("Failed to execute Python script");

        if output.status.success() {
            println!("TOTP secret generated and stored successfully.");
        } else {
            eprintln!("Failed to generate TOTP secret: {:?}", output.stderr);
        }
    } else {
        println!("Invalid account name. It must contain only letters and numbers.");
    }
}

fn create_sub_account_flow(conn: &mut diesel::PgConnection) {
    // Sub-account creation flow
    let mut currency = String::new();
    let mut balance = String::new();
    let mut subaccount_insert_account_id = String::new();

    // Get currency input
    print!("Enter currency (e.g., USD, EUR): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut currency).unwrap();
    let currency = currency.trim();

    if !validate_currency(currency) {
        println!("Invalid currency format. Please enter a valid currency code (e.g., USD, EUR).");
        return;
    }

    // Get balance input
    print!("Enter amount for the sub-account: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut balance).unwrap();
    let balance: f64 = match balance.trim().parse() {
        Ok(b) => b,
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
            return;
        }
    };

    // Get account ID input
    print!("Enter your account ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut subaccount_insert_account_id).unwrap();
    let subaccount_insert_account_id: Uuid = match subaccount_insert_account_id.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid account ID. Please enter a valid number.");
            return;
        }
    };

    // Here you would call your sub-account creation logic
    println!(
        "Creating sub-account with currency: {}, balance: {}, for account ID: {}",
        currency, balance, subaccount_insert_account_id
    );
    let sub_account =
        create_sub_account(conn, subaccount_insert_account_id, currency, balance).expect("Failed to create sub-account");
    println!("Sub-account created: {:#?}", sub_account);
}

fn login_flow(conn: &mut diesel::PgConnection) {
    // User login flow
    let mut username = String::new();
    let mut password = String::new();
    let mut totp_code = String::new();

    // Get username input
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    // Get password input
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    // Validate username and password (placeholder logic)
    if validate_user_credentials(conn, username, password) {
        // Get TOTP code input
        print!("Enter TOTP code: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut totp_code).unwrap();
        let totp_code = totp_code.trim();

        // Call the Python script to verify TOTP
        let output = std::process::Command::new("python")
            .arg("otp.py")
            .arg("verify")
            .arg(username)
            .arg(totp_code)
            .output()
            .expect("Failed to execute Python script");

        if output.status.success() {
            println!("Login successful.");
        } else {
            eprintln!("Login failed: Invalid TOTP code.");
        }
    } else {
        println!("Invalid username or password.");
    }
}

// Placeholder function for user credential validation
fn validate_user_credentials(conn: &mut diesel::PgConnection, username: &str, password: &str) -> bool {
    // Implement actual logic to validate username and password
    true
}

fn main() {
    dotenv().ok();  // Load environment variables from .env
    let mut conn = establish_connection();
    // loop {
    //     // Start the CLI loop by asking for user input
    //     println!("What can we do for you?");
    //     println!("1. Create Account");
    //     println!("2. Create Sub-Account");
    //     println!("3. Exit");
        
    //     // Get the user's choice
    //     let mut choice = String::new();
    //     print!("Enter your choice (1-3): ");
    //     io::stdout().flush().unwrap();
    //     io::stdin().read_line(&mut choice).unwrap();
    //     let choice = choice.trim();

    //     match choice {
    //         "1" => create_account_flow(&mut conn),
    //         "2" => create_sub_account_flow(&mut conn),
    //         "3" => {
    //             println!("Exiting... Goodbye!");
    //             break;
    //         }
    //         _ => println!("Invalid choice, please try again."),
    //     }
    // }

    // Create or get an account (if "John Doe" exists, it will return the existing one)
    let account_id_temp = create_account(&mut conn, "John Doe").unwrap();
    let account_id_temp2 = create_account(&mut conn, "Mr Singh").unwrap();

    // // // Create or get a sub-account with the specified currency for this account
    let sub_account_id_usd = create_sub_account(&mut conn, account_id_temp.id, "USD", 1000.0);
    let sub_account_id_usd2 = create_sub_account(&mut conn, account_id_temp2.id, "USD", 1000.0);

    // // // Trying to create the same sub-account with USD again won't create a duplicate
    // let sub_account_id_usd_duplicate = create_sub_account(&mut conn, account_id_temp.id, "USD", 1000.0);

    // // // Create another sub-account for the same account with a different currency
    let sub_account_id_eur = create_sub_account(&mut conn, account_id_temp.id, "EUR", 500.0);

    // let account_id_duplicate = create_account(&mut conn, "John Doe");

    let transaction1 = transfer_amount(&mut conn, account_id_temp.id, account_id_temp2.id, 100.0, "USD");
    
    // println!("Account: {:#?}", account_id);
    // println!("Sub-Account (USD) ID: {:#?}", sub_account_id_usd);
    // println!("Duplicate Sub-Account (USD) ID: {:#?}", sub_account_id_usd_duplicate);
    // println!("Sub-Account (EUR) ID: {:#?}", sub_account_id_eur);
    // println!("Account: {:#?}", account_id_duplicate);
}


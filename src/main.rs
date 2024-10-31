mod models;
pub mod schema;
mod database;

// use diesel::pg::PgConnection;
use dotenvy::dotenv;
// use std::env;
use crate::database::{establish_connection, create_account, create_sub_account, commit_transaction, get_balance};
use clap::{Parser, Subcommand};
use regex::Regex;
use std::io::{self, Write};
use uuid::Uuid;

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
    let account_id_temp1 = create_account(&mut conn, "John Doe");
    let account_id_temp1 = account_id_temp1.unwrap();
    let account_id_temp2 = create_account(&mut conn, "Charchit Aggarwal");
    let account_id_temp2 = account_id_temp2.unwrap();


    // // // Create or get a sub-account with the specified currency for this account
    let sub_account_id_usd1 = create_sub_account(&mut conn, account_id_temp1.id, "USD", 1000.0).expect("Failed to create sub-account");
    let sub_account_id_usd2 = create_sub_account(&mut conn, account_id_temp2.id, "USD", 1000.0).expect("Failed to create sub-account");

    // // // Create another sub-account for the same account with a different currency
    let sub_account_id_eur = create_sub_account(&mut conn, account_id_temp1.id, "EUR", 500.0);

    // let account_id_duplicate = create_account(&mut conn, "John Doe");

    // Test case 1: Successful transaction
    match commit_transaction(&mut conn, account_id_temp1.id, account_id_temp2.id, 100.0, "USD") {
        Ok(transaction) => println!("Transaction successful: {:#?}", transaction),
        Err(e) => println!("Transaction failed: {:?}", e),
    }
    let balance_temp1 = get_balance(&mut conn, account_id_temp1.id, "USD").expect("Failed to get balance");
    println!("Balance of account_id_temp1: {}", balance_temp1);
    let balance_temp2 = get_balance(&mut conn, account_id_temp2.id, "USD").expect("Failed to get balance");
    println!("Balance of account_id_temp2: {}", balance_temp2);
    
    // Test case 2: Insufficient balance
    match commit_transaction(&mut conn, account_id_temp1.id, account_id_temp2.id, 2000.0, "USD") {
        Ok(transaction) => println!("Transaction successful but should fail: {:#?}", transaction),
        Err(e) => println!("Transaction failed as expected (insufficient funds): {:?}", e),
    }
    let balance_temp1 = get_balance(&mut conn, account_id_temp1.id, "USD").expect("Failed to get balance");
    println!("Balance of account_id_temp1: {}", balance_temp1);
    let balance_temp2 = get_balance(&mut conn, account_id_temp2.id, "USD").expect("Failed to get balance");
    println!("Balance of account_id_temp2: {}", balance_temp2);
    
    // Test case 3: Invalid currency (EUR account doesn't exist for account_id_temp2)
    match commit_transaction(&mut conn, account_id_temp1.id, account_id_temp2.id, 100.0, "EUR") {
        Ok(transaction) => println!("Transaction successful but should fail: {:#?}", transaction),
        Err(e) => println!("Transaction failed as expected (invalid currency): {:?}", e),
    }
    let balance_temp1 = get_balance(&mut conn, account_id_temp1.id, "USD").expect("Failed to get balance");
    println!("Balance of account_id_temp1: {}", balance_temp1);
    let balance_temp2 = get_balance(&mut conn, account_id_temp2.id, "USD").expect("Failed to get balance");
    println!("Balance of account_id_temp2: {}", balance_temp2);
    
    // Test case 4: Transfer between same currency accounts
    match commit_transaction(&mut conn, account_id_temp2.id, account_id_temp1.id, 50.0, "USD") {
        Ok(transaction) => println!("Transaction successful: {:#?}", transaction),
        Err(e) => println!("Transaction failed: {:?}", e),
    }
    let balance_temp1 = get_balance(&mut conn, account_id_temp1.id, "USD").expect("Failed to get balance");
    println!("Balance of account_id_temp1: {}", balance_temp1);
    let balance_temp2 = get_balance(&mut conn, account_id_temp2.id, "USD").expect("Failed to get balance");
    println!("Balance of account_id_temp2: {}", balance_temp2);
    
    // println!("Account: {:#?}", account_id);
    // println!("Sub-Account (USD) ID: {:#?}", sub_account_id_usd);
    // println!("Duplicate Sub-Account (USD) ID: {:#?}", sub_account_id_usd_duplicate);
    // println!("Sub-Account (EUR) ID: {:#?}", sub_account_id_eur);
    // println!("Account: {:#?}", account_id_duplicate);
}


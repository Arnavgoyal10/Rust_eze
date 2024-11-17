mod models;
mod schema;
mod database;
mod moneytransfer;
mod otp_implement;

// use diesel::pg::PgConnection;
use dotenvy::dotenv;
// use std::env;
use crate::database::{establish_connection, create_account, create_sub_account, get_accounts, add_username_password, validate_username_password};
use crate::moneytransfer::{transfer_between_sub_accounts, get_balance, transfer_money, get_transactions, add_money_to_sub_account, approve_pending_transaction, get_pending_transactions};
use clap::{Parser, Subcommand};
use regex::Regex;
use std::io::{self, Write};
use uuid::Uuid;
use crate::otp_implement::{generate_totp_secret, verify_totp};

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
    let valid_currencies = vec!["USD", "EUR", "GBP", "JPY", "INR", "SGD", "EUR", "AUD"];
    if re.is_match(currency) {
        if valid_currencies.contains(&currency) {
            return true;
        }
    }
    false
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
        println!("Creating account: {}", account_name);
        //if account is created, ask for username and password and get account_id of created account and add it to username_password table
        match create_account(conn, account_name) {
            Ok(account) => {
                let account_id = account.id;
                let mut username = String::new();
                print!("Enter your username: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim();

                let mut password = String::new();
                print!("Enter your password: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut password).unwrap();
                let password = password.trim();


                match add_username_password(conn, &username, &password, account_id) {
                    Ok(_) => println!("Username and password added"),
                    Err(e) => println!("Failed to add username and password: {:?}", e),
                }
            }
            Err(e) => println!("Failed to create account: {:?}", e),    
        }
        
    } else {
        println!("Invalid account name. It must contain only letters and numbers.");
    }

    
}

fn create_sub_account_flow(conn: &mut diesel::PgConnection, subaccount_insert_account_id: Uuid) {
    // Sub-account creation flow
    let mut currency = String::new();
    let mut balance = String::new();

    // Get currency input
    print!("Enter currency (e.g., USD, EUR): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut currency).unwrap();
    let currency = currency.trim();

    if !validate_currency(currency) {
        println!("Invalid currency format. Please enter a valid currency code (e.g., USD, EUR).");
        return;
    }

    let balance = 0.0;

    println!(
        "Creating sub-account with currency: {}, balance: {}, for account ID: {}",
        currency, balance, subaccount_insert_account_id
    );
    match create_sub_account(conn, subaccount_insert_account_id, currency, balance) {
        Ok(sub_account) => println!("Sub-account created: {:#?}", sub_account),
        Err(e) => println!("Failed to create sub-account: {:?}", e),
    }
}

pub fn transfer_money_to_someone_else_flow(conn: &mut diesel::PgConnection, from_account_id: Uuid) {
    // Transfer money to someone else flow
    let mut to_account_id = String::new();
    let mut amount = String::new();
    let mut currency = String::new();

    // Get to account ID input
    print!("Enter the recipient's account ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut to_account_id).unwrap();
    let to_account_id: Uuid = match to_account_id.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid account ID. Please enter a valid number.");
            return;
        }
    };
    // Get currency input
    print!("Enter the currency to transfer: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut currency).unwrap();
    let currency = currency.trim();

    if !validate_currency(currency) {
        println!("Invalid currency format. Please enter a valid currency code (e.g., USD, EUR).");
        return;
    }

    // Get amount input
    print!("Enter the amount to transfer: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).unwrap();
    let amount: f64 = match amount.trim().parse() {
        Ok(a) => a,
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
            return;
        }
    };

    if !validate_amount(amount) {
        println!("Invalid amount. Please enter a valid number.");
        return;
    }

    println!("Transferring {} {} to account {} from account {}", amount, currency, to_account_id, from_account_id);

    match transfer_money(conn, from_account_id, to_account_id, amount, currency) {
        Ok(transaction) => println!("Transaction successful: {:#?}", transaction),
        Err(e) => println!("Transaction failed: {:?}", e),
    }
}

pub fn transfer_between_sub_accounts_flow(conn: &mut diesel::PgConnection, from_account_id: Uuid) {
    // Transfer between sub-accounts flow
    let mut amount = String::new();
    let mut from_currency = String::new();  
    let mut to_currency = String::new();

    // Get from currency input
    print!("Enter the currency of the sub-account to transfer from: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut from_currency).unwrap();
    let from_currency = from_currency.trim();

    if !validate_currency(from_currency) {
        println!("Invalid currency format. Please enter a valid currency code (e.g., USD, EUR).");
        return;
    }

    // Get to currency input
    print!("Enter the currency of the sub-account to transfer to: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut to_currency).unwrap();
    let to_currency = to_currency.trim();

    if !validate_currency(to_currency) {
        println!("Invalid currency format. Please enter a valid currency code (e.g., USD, EUR).");
        return;
    }

    // Get amount input
    print!("Enter the amount to transfer: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).unwrap();
    let amount: f64 = match amount.trim().parse() {
        Ok(a) => a,
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
            return;
        }
    };

    if !validate_amount(amount) {
        println!("Invalid amount. Please enter a valid number.");
        return;
    }

    println!("Converting {} {} to {} in account {}", amount, from_currency, to_currency, from_account_id);

    match transfer_between_sub_accounts(conn, from_account_id, from_currency, to_currency, amount) {
        Ok(transaction) => println!("Transaction successful: {:#?}", transaction),
        Err(e) => println!("Transaction failed: {:?}", e),
    }

}

fn get_balance_flow(conn: &mut diesel::PgConnection, from_account_id: Uuid) {
    // Get balance flow
    let mut currency = String::new();

    // Get currency input
    print!("Enter the currency to get the balance for: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut currency).unwrap();
    let currency = currency.trim();

    println!("Getting balance for account {} in currency {}", from_account_id, currency);

    match get_balance(conn, from_account_id, currency) {
        Ok(balance) => println!("Balance: {}", balance),
        Err(e) => println!("Failed to get balance: {:?}", e),
    }
}

fn get_transactions_flow(conn: &mut diesel::PgConnection, account_id: Uuid) {

    match get_transactions(conn, account_id) {
        Ok(transactions) => println!("Transactions: {:#?}", transactions),
        Err(e) => println!("Failed to get transactions: {:?}", e),
    }
}

pub fn get_accounts_flow(conn: &mut diesel::PgConnection) {
    match get_accounts(conn) {
        Ok(accounts) => println!("Accounts: {:#?}", accounts),
        Err(e) => println!("Failed to get accounts: {:?}", e),
    }
}

pub fn validate_account_id(account_id: Uuid, conn: &mut diesel::PgConnection) -> bool {
    use diesel::prelude::*;
    use crate::schema::accounts::dsl::*;
    use crate::models::Account;
    // Use diesel's query interface instead of a non-existent find method
    match accounts.find(account_id).first::<Account>(conn) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn get_pending_transactions_flow(conn: &mut diesel::PgConnection) {
    match get_pending_transactions(conn) {
        Ok(pending_transactions) => println!("Pending transactions: {:#?}", pending_transactions),
        Err(e) => println!("Failed to get pending transactions: {:?}", e),
    }
}

pub fn login_flow(conn: &mut diesel::PgConnection) {
    let mut username = String::new();
    let mut password = String::new();   
    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();
    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    // Get the account_id and handle the Option
    match validate_username_password(conn, &username, &password) {
        Some(account_id) => {
            // Start the CLI loop
            loop {
                println!("What can we do for you?");
                println!("1. Create Sub-Account");
                println!("2. Transfer between sub-accounts");
                println!("3. Transfer money to someone else");
                println!("4. Get balance");
                println!("5. Get transactions");
                println!("6. Add money to sub-account");
                println!("7. Exit");
                
                let mut choice = String::new();
                print!("Enter your choice (1-7): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut choice).unwrap();
                let choice = choice.trim();

                match choice {
                    "1" => create_sub_account_flow(conn, account_id),
                    "2" => transfer_between_sub_accounts_flow(conn, account_id),
                    "3" => transfer_money_to_someone_else_flow(conn, account_id),
                    "4" => get_balance_flow(conn, account_id),
                    "5" => get_transactions_flow(conn, account_id),
                    "6" => add_money_to_sub_account_flow(conn, account_id),
                    "7" => {
                        println!("Exiting... Goodbye!");
                        break;
                    }
                    _ => println!("Invalid choice, please try again."),
                }
            }
        },
        None => println!("Invalid username or password"),
    }
}

pub fn admin_flow(conn: &mut diesel::PgConnection) {
    let mut username = String::new();
    let mut password = String::new();
    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();
    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    if username == "admin" && password == "6969" {
        loop {
            println!("=== ADMIN MODE ===");
            println!("1. Get pending transactions");
            println!("2. Approve pending transaction");
            println!("3. Get all accounts");
            println!("4. Exit");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();
            match choice {
                "1" => get_pending_transactions_flow(conn),
                "2" => approve_pending_transaction_flow(conn),
                "3" => get_accounts_flow(conn),
                "4" => {
                    println!("Exiting... Goodbye!");
                    break;
                }
                _ => println!("Invalid choice, please try again."),
            }
        }
    } else {
        println!("Invalid username or password");
    }
}

pub fn approve_pending_transaction_flow(conn: &mut diesel::PgConnection) {
    let mut pending_transaction_id = String::new();
    print!("Enter the ID of the pending transaction to approve: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pending_transaction_id).unwrap();
    let pending_transaction_id: Uuid = match pending_transaction_id.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid pending transaction ID. Please enter a valid number.");
            return;
        }
    };
    
    match approve_pending_transaction(conn, pending_transaction_id) {
        Ok(_) => println!("Pending transaction approved"),
        Err(e) => println!("Failed to approve pending transaction: {:?}", e),
    }
}

pub fn add_money_to_sub_account_flow(conn: &mut diesel::PgConnection, account_id: Uuid) {
    let mut amount = String::new();
    print!("Enter the amount to add: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).unwrap();
    let amount: f64 = match amount.trim().parse() {
        Ok(a) => a,
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
            return;
        }
    };
    let mut currency = String::new();
    print!("Enter the currency to add: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut currency).unwrap();
    let currency = currency.trim();
    match add_money_to_sub_account(conn, account_id, amount, currency) {
        Ok(_) => println!("Money added to sub-account"),
        Err(e) => println!("Failed to add money to sub-account: {:?}", e),
    }
}


fn main() {
    dotenv().ok();
    let mut conn = establish_connection();
    loop {
        println!("Welcome to the Account Manager");
        println!("1. Login");
        println!("2. Create Account");
        println!("3. Get all accounts");
        println!("4. Admin Mode");
        println!("5. Exit");
        println!("Enter your choice (1-5): ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        match choice {
            "1" => login_flow(&mut conn),
            "2" => create_account_flow(&mut conn),  
            "3" => get_accounts_flow(&mut conn),
            "4" => admin_flow(&mut conn),
            "5" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }   
    }
}

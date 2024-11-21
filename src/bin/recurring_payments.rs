use rust_eze::recurringpayments::{establish_connection, process_scheduled_transactions, log_to_file};
use tokio;
#[tokio::main]
async fn main() {
    let mut conn = establish_connection();
    if let Err(e) = process_scheduled_transactions(&mut conn).await {
        log_to_file(&format!("Error processing scheduled transactions: {:?}", e));
        std::process::exit(1);
    }
} 
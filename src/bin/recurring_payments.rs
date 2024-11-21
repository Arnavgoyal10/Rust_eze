use rust_eze::recurringpayments::{establish_connection, process_scheduled_transactions, log_to_file};
use tokio;
#[tokio::main]
async fn main() {
    log_to_file("Starting recurring payments process");
    let mut conn = establish_connection();
    match process_scheduled_transactions(&mut conn).await {
        Ok(_) => log_to_file("Scheduled transactions processed successfully"),
        Err(e) => log_to_file(&format!("Error processing scheduled transactions: {:?}", e)),
    }
    log_to_file("Recurring payments process completed");
} 
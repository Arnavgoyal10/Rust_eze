use chrono::{NaiveDateTime, Datelike, Utc, NaiveDate};
use diesel::prelude::*;
use crate::moneytransfer::transfer_money;
use crate::models::ScheduledTransaction;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use dotenvy::dotenv;
use crate::sendalert::send_telegram_alert;
use std::env;

   
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub async fn process_scheduled_transactions(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::scheduled_transactions::dsl::*;
    let time_to_filter_by = Utc::now().naive_utc().date().and_hms_opt(0, 0, 0).unwrap();
    //println!("Time to filter by: {:?}", time_to_filter_by);
    // Filter for transactions scheduled for today
    let pending_transactions: Vec<ScheduledTransaction> = scheduled_transactions
        .filter(executed.eq(false))
        .filter(scheduled_date.eq(time_to_filter_by)) 
        .load(conn)?;

    for transaction in pending_transactions {
        // Execute the transfer
        match transfer_money(
            conn,
            transaction.from_account_id,
            transaction.to_account_id,
            transaction.amount,
            &transaction.currency,
        ) {
            Ok(_) => {
                let current_date = transaction.scheduled_date;
                let current_month = current_date.month();
                let current_year = current_date.year();
                let next_month = if current_month == 12 {
                    1 // January
                } else {
                    current_month + 1
                };
                
                // Calculate the next year if the month rolls over
                let next_year = if next_month == 1 {
                    current_year + 1
                } else {
                    current_year
                };

                let day = current_date.day();
                let last_day_of_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap().pred_opt().unwrap().day();
                let next_day = if day > last_day_of_next_month {last_day_of_next_month} else {day};

                let next_date = NaiveDateTime::new(NaiveDate::from_ymd_opt(next_year, next_month, next_day).unwrap(), current_date.time());
                // Mark the scheduled transaction as executed
                diesel::update(scheduled_transactions.find(transaction.id))
                    .set(scheduled_date.eq(next_date))
                    .execute(conn)?;

                send_telegram_alert(&format!("Executed scheduled transaction: {:?}", transaction)).await;
                //println!("Executed scheduled transaction: {:?}", transaction);
            }
            Err(e) => {
                send_telegram_alert(&format!("Failed to execute scheduled transaction {:?}: {:?}", transaction, e)).await;
                println!("Failed to execute scheduled transaction {:?}: {:?}", transaction, e);
            }
        }
    }

    Ok(())
}



pub fn log_to_file(message: &str) {
    let log_path = "recurring_payments.log";
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path) 
    {
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}
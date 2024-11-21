use teloxide::prelude::*;
use crate::recurringpayments::log_to_file;

pub async fn send_telegram_alert(message: &str) {
    log_to_file("Starting telegram alert function");
    
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set");
    let chat_id = std::env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID must be set")
        .parse::<i64>()
        .expect("TELEGRAM_CHAT_ID must be a valid integer");
    
    let bot = Bot::new(bot_token);
    
    match bot.send_message(ChatId(chat_id), message).await {
        Ok(_) => log_to_file("Telegram alert sent successfully"),
        Err(e) => {
            log_to_file(&format!("Failed to send Telegram alert: {:#?}", e));
            panic!("Failed to send Telegram message: {:#?}", e);
        }
    }
    
    log_to_file("Telegram alert function completed");
}
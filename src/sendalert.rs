use telegram_bot::{Api, SendMessage, ChatId};
use crate::recurringpayments::log_to_file;

pub async fn send_telegram_alert(message: &str) {
    let chat_id = std::env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID must be set")
        .parse::<i64>()
        .expect("TELEGRAM_CHAT_ID must be a valid integer");
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set");
        
    let api = Api::new(&bot_token);
    let message = SendMessage::new(ChatId::new(chat_id), message);
    
    match api.send(message).await {
        Ok(_) => println!("Telegram alert sent successfully"),
        Err(e) => {
            println!("Failed to send Telegram message: {:?}", e);
            log_to_file(&format!("Failed to send Telegram alert: {}", e));
        }
    }
}
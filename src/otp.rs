use otpauth::TOTP;
use qrcode::QrCode;
use qrcode::render::unicode;
use rand::Rng;
use base32::{Alphabet, encode};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, FixedOffset, TimeZone};

/// Helper function to get the current timestamp in seconds
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Helper function to convert timestamp to a human-readable datetime in IST (UTC+5:30)
fn convert_to_datetime_ist(timestamp: u64) -> DateTime<FixedOffset> {
    let naive = chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
        .expect("Invalid timestamp");
    // UTC+5:30 for India Standard Time (IST)
    let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();
    DateTime::<FixedOffset>::from_utc(naive, ist_offset)
}

/// Generate a random Base32 encoded secret for TOTP (without padding)
pub fn generate_secret() -> String {
    let random_bytes: [u8; 10] = rand::thread_rng().gen();
    encode(Alphabet::RFC4648 { padding: false }, &random_bytes)  // Base32 encode without padding
}

/// Create a new TOTP instance using the given secret key
pub fn create_totp(secret: &str) -> TOTP {
    TOTP::new(secret)
}

/// Generate the expected OTP for the current time
pub fn generate_otp(totp: &TOTP) -> u32 {
    let timestamp = current_timestamp();
    let otp = totp.generate(30, timestamp);  // 30-second window for OTP
    otp
}

/// Verify if the provided OTP is valid for the current time
pub fn verify_otp(totp: &TOTP, user_otp: &str) -> bool {
    let timestamp = current_timestamp();
    let datetime_ist = convert_to_datetime_ist(timestamp);  // Convert the timestamp to IST
    println!("Current Timestamp (IST): {}", datetime_ist);  // Print the datetime in IST

    let expected_otp = generate_otp(totp);
    println!("Expected OTP: {}", expected_otp);  // Debugging: Print the expected OTP

    match user_otp.parse::<u32>() {
        Ok(parsed_otp) => {
            println!("Verifying OTP: {}", parsed_otp);  // Debugging: Print the OTP being verified
            let is_valid = expected_otp == parsed_otp;
            println!("Is OTP valid? {}", is_valid);
            is_valid
        },
        Err(_) => {
            println!("Failed to parse OTP: {}", user_otp);  // Debugging: Log failed parsing
            false // Return false if OTP parsing fails
        },
    }
}

/// Generate a QR code that can be scanned by an authenticator app
pub fn generate_qr_code(_totp: &TOTP, secret: &str) {
    // The provisioning URL for the QR code, encoding the secret without padding
    let provisioning_url = format!(
        "otpauth://totp/MyService:user@example.com?secret={}&issuer=MyService",
        secret
    );
    println!("Provisioning URL: {}", provisioning_url);  // Debugging: Print the provisioning URL
    
    let qr_code_image = QrCode::new(provisioning_url).unwrap();
    let image = qr_code_image.render::<unicode::Dense1x2>().build();
    println!("Generating QR Code with Secret: {}", secret);
    println!("Scan this QR code to set up your authenticator:\n{}", image);
}

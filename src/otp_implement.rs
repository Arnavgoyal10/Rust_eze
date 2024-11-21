use std::process::Command;
use std::str;
use diesel::PgConnection;
use anyhow::{Result, anyhow};


pub fn generate_totp_secret() -> Result<String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| anyhow!("Failed to get current directory: {}", e))?;
    let script_path = current_dir.join("src").join("otp.py");

    let output = Command::new("python3")
        .arg(&script_path)
        .arg("generate")
        .output()
        .map_err(|e| anyhow!("Failed to execute Python script: {}", e))?;

    if !output.status.success() {
        let error = str::from_utf8(&output.stderr)
            .unwrap_or("Unknown error")
            .trim();
        return Err(anyhow!("Python script failed: {}", error));
    }

    let secret = str::from_utf8(&output.stdout)
        .map_err(|e| anyhow!("Failed to parse output: {}", e))?
        .trim()
        .to_string();

    Ok(secret)
}

pub fn verify_totp(secret: &str, totp_code: &str) -> Result<bool> {
    let current_dir = std::env::current_dir()
        .map_err(|e| anyhow!("Failed to get current directory: {}", e))?;
    let script_path = current_dir.join("src").join("otp.py");

    let output = Command::new("python3")
        .arg(&script_path)
        .arg("verify")
        .arg(secret)
        .arg(totp_code)
        .output()
        .map_err(|e| anyhow!("Failed to execute Python script: {}", e))?;

    if !output.status.success() {
        let error = str::from_utf8(&output.stderr)
            .unwrap_or("Unknown error")
            .trim();
        return Err(anyhow!("Python script failed: {}", error));
    }

    let result = str::from_utf8(&output.stdout)
        .map_err(|e| anyhow!("Failed to parse output: {}", e))?
        .trim();

    Ok(result == "true")
}

pub fn verify_totp_flow(conn: &mut PgConnection, username_to_verify: &str) -> Result<bool> {
    use crate::schema::username_password::dsl::*;
    use crate::models::UsernamePassword;
    use diesel::prelude::*;
    
    // Load the user data first
    let user_data = username_password
        .filter(username.eq(username_to_verify))
        .load::<UsernamePassword>(conn)?;
    
    // Then get the TOTP secret
    let secret = user_data
        .get(0)
        .and_then(|up| up.totp_secret.as_ref())
        .ok_or_else(|| anyhow!("User not found or no TOTP secret set"))?;

    //take totp code from the user
    println!("Enter the TOTP code: ");
    let mut totp_code = String::new();
    std::io::stdin().read_line(&mut totp_code)?;
    totp_code = totp_code.trim().to_string();   

    if verify_totp(&secret, &totp_code)? {
        Ok(true)
    } else {
        Ok(false)
    }
}

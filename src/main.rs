use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;
mod otp;

// A shared state to store the secret across requests
struct AppState {
    secret: Mutex<String>, // Use a Mutex to ensure safe concurrent access
}

#[derive(Deserialize)]
struct OtpRequest {
    otp: String,
}

async fn verify_otp_endpoint(data: web::Data<AppState>, req: web::Json<OtpRequest>) -> impl Responder {
    let secret = data.secret.lock().unwrap(); // Access the secret from state
    let totp = otp::create_totp(&secret);

    if otp::verify_otp(&totp, &req.otp) {
        HttpResponse::Ok().body("OTP is valid!")
    } else {
        HttpResponse::Unauthorized().body("Invalid OTP")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting OTP authentication service...");

    // Generate a secret and store it in application state
    let secret = otp::generate_secret();
    println!("Your Secret Key: {}", secret);

    // Create a TOTP instance using the generated secret
    let totp = otp::create_totp(&secret);

    // Print the QR code to the console
    otp::generate_qr_code(&totp, &secret);

    // Store the secret in shared application state
    let shared_secret = web::Data::new(AppState {
        secret: Mutex::new(secret),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_secret.clone()) // Share state with the server
            .route("/verify", web::post().to(verify_otp_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

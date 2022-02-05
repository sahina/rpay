use std::env::var;

use dotenv::dotenv;

pub fn database_url() -> String {
    dotenv().ok();
    var("DATABASE_URL").expect("DATABASE_URL is not set")
}

pub fn secret_key() -> String {
    dotenv().ok();
    var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8))
}

pub fn domain() -> String {
    dotenv().ok();
    var("DOMAIN").unwrap_or_else(|_| "localhost".to_string())
}

pub fn port() -> u16 {
    dotenv().ok();
    var("PORT").expect("PORT is not set").parse::<u16>().ok().expect("PORT should be an integer")
}

pub fn domain_url() -> String {
    dotenv().ok();
    var("DOMAIN_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

pub fn smtp_username() -> String {
    dotenv().ok();
    var("SMTP_USERNAME").expect("SMTP_USERNAME is not set")
}

pub fn smtp_password() -> String {
    dotenv().ok();
    var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set")
}

pub fn smtp_host() -> String {
    dotenv().ok();
    var("SMTP_HOST").expect("SMTP_HOST is not set")
}

pub fn smtp_port() -> u16 {
    dotenv().ok();
    var("SMTP_PORT").expect("SMTP_PORT is not set").parse::<u16>().ok().expect("SMTP_PORT should be an integer")
}

pub fn smtp_sender_name() -> String {
    dotenv().ok();
    var("SMTP_SENDER_NAME").expect("SMTP_SENDER_NAME is not set")
}

pub fn plivo_auth_id() -> String {
    dotenv().ok();
    var("PLIVO_AUTH_ID").expect("PLIVO_AUTH_ID is not set")
}

pub fn plivo_token() -> String {
    dotenv().ok();
    var("PLIVO_TOKEN").expect("PLIVO_TOKEN is not set")
}

pub fn templates_glob() -> String {
    "templates/*.html".to_string()
}

pub fn nats_url() -> String {
    dotenv().ok();
    var("NATS_URL").expect("NATS_URL is not set")
}
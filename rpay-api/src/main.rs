use std::net::TcpListener;

use op_api::startup::run;
use rpay_core::log::setup_trace;
use rpay_core::vars::{domain, port};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_trace();
    dotenv::dotenv().ok();

    let port = port();
    let host = domain();
    let address = format!("{}:{}", host, port);

    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}

use std::net::TcpListener;

use actix_http::Method;
use actix_web::{App, HttpServer, middleware, web};
use actix_web::dev::Server;
use nats;
use nats::Connection;
use tracing::info;

use rpay_core::vars::nats_url;

use crate::api::routes::site::{health_check, landing};

#[derive(Debug, Clone)]
pub struct AppData {
    pub nc: Connection,
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    info!("Server started at: {:?}", &listener.local_addr().unwrap());

    let app_data = AppData {
        nc: nats::connect(nats_url().as_str())?
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::method(Method::GET).to(landing))
            .route("/health", web::method(Method::GET).to(health_check))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
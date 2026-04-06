use crate::application::BankService;
use crate::data::InMemoryAccountRepository;
use crate::infrastructure::{Config, init_logging};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use tracing::info;
use crate::presentation::middleware::RequestIdMiddleware;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    init_logging();

    let cfg = Config::from_env().expect("Invalid configuration");

    let acc_repo = Arc::new(InMemoryAccountRepository::default());
    let service = BankService::new(acc_repo.clone());

    let addr = format!("{}:{}", cfg.host, cfg.port);
    info!("→ listening on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&cfg.cors_origin)
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::AUTHORIZATION,
                    ])
                    .supports_credentials()
                    .max_age(600),
            )
            .wrap(RequestIdMiddleware)
            .wrap(Logger::default())
            .app_data(web::Data::new(service.clone()))
            .app_data(cfg.clone())
            .configure(presentation::routes::configure)
    })
    .bind(addr)?
    .run()
    .await
}

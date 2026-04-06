use crate::application::BankService;
use crate::presentation::dto::{AccountResponse, CreateAccountRequest};
use actix_web::{HttpResponse, Responder, Result, get, post, web};
use actix_web::web::service;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello, World!"))
}

#[post("/accounts")]
async fn create_account(
    service: web::Data<BankService>,
    payload: web::Json<CreateAccountRequest>,
) -> Result<impl Responder> {
    service
        .create_account(payload.id, payload.initial_balance)
        .await?;

    let account = service.get_account(payload.id).await?;

    Ok(HttpResponse::Created().json(AccountResponse::from(account)))
}

#[get("/accounts/{id}")]
async fn get_balance(
    service: web::Data<BankService>,
    payload: web::Path<u32>
) -> Result<impl Responder> {
    let acc_id = payload.into_inner();
    let balance = service.get_balance(acc_id).await?;
    Ok(HttpResponse::Ok().json(format!("Balance: {}", balance)))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index)
        .service(create_account)
        .service(get_balance)
    ;
}

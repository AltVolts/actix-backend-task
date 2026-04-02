use crate::application::BankService;
use crate::data::InMemoryAccountRepository;
use crate::domain::DomainError;
use crate::presentation::dto::{AccountResponse, ApiError, CreateAccountRequest};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, ResponseError, Result, get, post, web};

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello, World!"))
}

#[post("/accounts")]
async fn create_account(
    service: web::Data<BankService<InMemoryAccountRepository>>,
    payload: web::Json<CreateAccountRequest>,
) -> Result<impl Responder> {
    service
        .create_account(payload.id, payload.initial_balance)
        .await?;

    let account = service.get_account(payload.id).await?;

    Ok(HttpResponse::Created().json(AccountResponse::from(account)))
}

impl ResponseError for DomainError {
    fn status_code(&self) -> StatusCode {
        match self {
            DomainError::AccountNotFound => StatusCode::NOT_FOUND,
            DomainError::InvalidAmount(_) => StatusCode::BAD_REQUEST,
            DomainError::AccountAlreadyExists => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            DomainError::InvalidAmount(msg) => HttpResponse::BadRequest().json(ApiError::new(msg)),
            DomainError::AccountNotFound => {
                HttpResponse::NotFound().json(ApiError::new("account not found"))
            }
            DomainError::AccountAlreadyExists => {
                HttpResponse::Conflict().json(ApiError::new("account already exists"))
            }
        }
    }
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index).service(create_account);
}

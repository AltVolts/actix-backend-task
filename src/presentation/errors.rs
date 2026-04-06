use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use crate::domain::DomainError;

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            DomainError::AccountNotFound => StatusCode::NOT_FOUND,
            DomainError::InvalidAmount(_) => StatusCode::BAD_REQUEST,
            DomainError::AccountAlreadyExists => StatusCode::CONFLICT,
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        }))
    }
}
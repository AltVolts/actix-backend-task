use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid amount: {0}")]
    InvalidAmount(String),
    #[error("account not found")]
    AccountNotFound,
    #[error("account already exists")]
    AccountAlreadyExists,
}

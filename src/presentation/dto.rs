use crate::domain::Account;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub id: u32,
    pub initial_balance: i64,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: u32,
    pub balance: i64,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            balance: account.balance,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { error: msg.into() }
    }
}

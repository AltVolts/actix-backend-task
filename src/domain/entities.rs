use crate::domain::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u32,
    pub balance: i64,
}

impl Account {
    pub fn new(id: u32, initial_balance: i64) -> Result<Self, DomainError> {
        if initial_balance < 0 {
            return Err(DomainError::InvalidAmount(
                "Initial balance must be non-negative".into(),
            ));
        }

        Ok(Self {
            id,
            balance: initial_balance,
        })
    }
}

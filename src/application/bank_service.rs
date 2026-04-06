use crate::domain::{Account, AccountRepository, DomainError};
use std::sync::Arc;

#[derive(Clone)]
pub struct BankService {
    repo: Arc<dyn AccountRepository>,
}

impl BankService {
    pub fn new(repo: Arc<dyn AccountRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_account(&self, id: u32, initial_balance: i64) -> Result<(), DomainError> {
        let account = Account::new(id, initial_balance)?;
        self.repo.create(account).await
    }

    pub async fn get_account(&self, id: u32) -> Result<Account, DomainError> {
        match self.repo.get(id).await? {
            Some(account) => Ok(account),
            None => Err(DomainError::AccountNotFound),
        }
    }

    pub async fn get_balance(&self, id: u32) -> Result<i64, DomainError> {
        let account = self.get_account(id).await?;
        Ok(account.balance)
    }
}

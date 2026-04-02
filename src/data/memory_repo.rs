use crate::domain::{Account, AccountRepository, DomainError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct InMemoryAccountRepository {
    accounts: Arc<RwLock<HashMap<u32, Account>>>,
}

impl AccountRepository for InMemoryAccountRepository {
    async fn create(&self, account: Account) -> Result<(), DomainError> {
        let mut accounts = self.accounts.write().await;
        if accounts.contains_key(&account.id) {
            return Err(DomainError::AccountAlreadyExists);
        }
        accounts.insert(account.id, account);
        Ok(())
    }

    async fn get(&self, id: u32) -> Result<Option<Account>, DomainError> {
        let accounts = self.accounts.read().await;
        Ok(accounts.get(&id).cloned())
    }

    async fn upsert(&self, account: Account) -> Result<(), DomainError> {
        let mut accounts = self.accounts.write().await;
        accounts.insert(account.id, account);
        Ok(())
    }
}

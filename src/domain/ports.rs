use super::entities::Account;
use super::errors::DomainError;

pub trait AccountRepository: Send + Sync {
    async fn create(&self, account: Account) -> Result<(), DomainError>;
    async fn get(&self, id: u32) -> Result<Option<Account>, DomainError>;
    async fn upsert(&self, account: Account) -> Result<(), DomainError>;
}

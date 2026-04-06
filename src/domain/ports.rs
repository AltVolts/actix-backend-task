use async_trait::async_trait;
use super::entities::Account;
use super::errors::DomainError;

#[async_trait]
pub trait AccountRepository: Send + Sync + 'static {
    async fn create(&self, account: Account) -> Result<(), DomainError>;
    async fn get(&self, id: u32) -> Result<Option<Account>, DomainError>;
    async fn upsert(&self, account: Account) -> Result<(), DomainError>;
}

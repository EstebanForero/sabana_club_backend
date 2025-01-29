use async_trait::async_trait;
use mockall::automock;

pub mod err;
use err::Result;
pub mod lib_sql_implementation;

#[automock]
#[async_trait]
pub trait UniqueIdentifierRepository: Send + Sync {
    async fn get_user_id_by_email(&self, email: &str) -> Result<String>;
    async fn get_user_id_by_phone_number(&self, phone_number: &str) -> Result<String>;
    async fn comprove_id_existance(&self, user_id: &str) -> Result<()>;
}

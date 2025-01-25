use super::domain::UserCreationInfo;

mod err;
use async_trait::async_trait;
use err::Result;
use mockall::automock;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user_creation_info: UserCreationInfo) -> Result<()>;
    async fn get_user_id_by_email(&self, email: &String) -> Result<String>;
    async fn get_user_id_by_phone_number(&self, phone_number: &String) -> Result<String>;
}

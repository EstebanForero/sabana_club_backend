use super::domain::UserCreationInfo;

pub mod err;
use super::domain::UserInfo;
use async_trait::async_trait;
use err::Result;
use mockall::automock;

pub mod libsql_implementation;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user_creation_info: UserCreationInfo) -> Result<()>;
    async fn get_user_id_by_email(&self, email: &String) -> Result<String>;
    async fn get_user_id_by_phone_number(&self, phone_number: &String) -> Result<String>;
    async fn get_user_password(&self, user_id: &String) -> Result<String>;

    async fn get_users(&self) -> Result<Vec<UserInfo>>;
    async fn get_user_by_id(&self, user_id: &String) -> Result<UserInfo>;
}

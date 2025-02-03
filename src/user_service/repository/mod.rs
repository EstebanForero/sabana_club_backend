use super::domain::SearchSelection;
use super::domain::UserCreationInfo;

pub mod err;
use super::domain::UserInfo;
use super::domain::UserRol;
use super::domain::UserSelectionInfo;
use super::domain::UserUpdating;
use async_trait::async_trait;
use err::Result;
use mockall::automock;

pub mod libsql_implementation;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user_creation_info: UserCreationInfo) -> Result<()>;
    async fn get_user_password(&self, user_id: &String) -> Result<String>;

    async fn get_users(&self) -> Result<Vec<UserInfo>>;
    async fn get_user_by_id(&self, user_id: &String) -> Result<UserInfo>;

    async fn user_rol(&self, user_id: &str) -> Result<UserRol>;

    async fn modify_user(&self, updated_user_info: UserUpdating, user_id: &str) -> Result<()>;

    async fn search_users_by_search_selection(
        &self,
        search: &str,
        limit: u8,
        search_parameter: SearchSelection,
    ) -> Result<Vec<UserSelectionInfo>>;
}

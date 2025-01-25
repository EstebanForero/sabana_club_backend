use super::domain::UserCreationInfo;

mod err;
use err::Result;

pub trait UserRepository {
    async fn create_user(&self, user_creation_info: UserCreationInfo);
    fn get_user_id_by_email(&self, email: String) -> Result<String>;
    fn get_user_id_by_phone_number(&self, phone_number: String) -> Result<String>;
}

use std::sync::Arc;

use async_trait::async_trait;
use tracing::error;

use super::repository::UserRepository;

#[async_trait]
trait UniqueIdentifier {
    async fn identify(&self, identification_token: String) -> Option<String>;
}

struct PhoneIdentifier {
    user_repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl UniqueIdentifier for PhoneIdentifier {
    async fn identify(&self, identification_token: String) -> Option<String> {
        if !identification_token
            .chars()
            .into_iter()
            .any(|char| !char.is_numeric())
        {
            let user_id = self
                .user_repository
                .get_user_id_by_phone_number(&identification_token)
                .await;

            match user_id {
                Ok(user_id) => return Some(user_id),
                Err(err) => {
                    error!("Error getting user id with the email <{identification_token}>, error: {err}")
                }
            }
        }

        None
    }
}

struct EMailIdentifier {
    user_repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl UniqueIdentifier for EMailIdentifier {
    async fn identify(&self, identification_token: String) -> Option<String> {
        if identification_token.contains("@") && identification_token.contains(".") {
            let user_id = self
                .user_repository
                .get_user_id_by_email(&identification_token)
                .await;

            match user_id {
                Ok(user_id) => return Some(user_id),
                Err(err) => {
                    error!("Error getting user id with the email <{identification_token}>, error: {err}")
                }
            }
        }

        None
    }
}

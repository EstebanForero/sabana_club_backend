use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use tracing::{error, info};
use uuid::Uuid;

use super::repository::UniqueIdentifierRepository;

pub fn build_unique_identifier(
    user_repository: Arc<dyn UniqueIdentifierRepository>,
) -> Arc<dyn UniqueIdentifier> {
    let phone_identifier: Arc<dyn UniqueIdentifier> =
        Arc::new(PhoneIdentifier::new(user_repository.clone(), None));

    let email_identifier: Arc<dyn UniqueIdentifier> = Arc::new(EMailIdentifier::new(
        user_repository.clone(),
        Some(phone_identifier),
    ));

    let unique_identifier: Arc<dyn UniqueIdentifier> =
        Arc::new(UserIdentifier::new(Some(email_identifier)));

    unique_identifier
}

#[async_trait]
pub trait UniqueIdentifier: Sync + Send {
    async fn identify(&self, identification_token: String) -> Option<String>;

    fn next(&self) -> Option<Arc<dyn UniqueIdentifier>>;
}

pub struct PhoneIdentifier {
    user_repository: Arc<dyn UniqueIdentifierRepository>,
    next_identifier: Option<Arc<dyn UniqueIdentifier>>,
}

impl PhoneIdentifier {
    pub fn new(
        user_repository: Arc<dyn UniqueIdentifierRepository>,
        next_identifier: Option<Arc<dyn UniqueIdentifier>>,
    ) -> Self {
        PhoneIdentifier {
            user_repository,
            next_identifier,
        }
    }
}

#[async_trait]
impl UniqueIdentifier for PhoneIdentifier {
    async fn identify(&self, identification_token: String) -> Option<String> {
        info!("Executing unique phone identifier");
        if identification_token.chars().all(|char| char.is_numeric()) {
            let user_id = self
                .user_repository
                .get_user_id_by_phone_number(&identification_token)
                .await;

            match user_id {
                Ok(user_id) => {
                    return Some(user_id);
                }
                Err(err) => {
                    error!(
                        "Error getting user id with phone number <{identification_token}>, error: {err}"
                    );
                }
            }
        }

        if let Some(next_identifier) = &self.next_identifier {
            next_identifier.identify(identification_token).await
        } else {
            None
        }
    }

    fn next(&self) -> Option<Arc<dyn UniqueIdentifier>> {
        self.next_identifier.clone()
    }
}

pub struct EMailIdentifier {
    user_repository: Arc<dyn UniqueIdentifierRepository>,
    next_identifier: Option<Arc<dyn UniqueIdentifier>>,
}

impl EMailIdentifier {
    pub fn new(
        user_repository: Arc<dyn UniqueIdentifierRepository>,
        next_identifier: Option<Arc<dyn UniqueIdentifier>>,
    ) -> Self {
        EMailIdentifier {
            user_repository,
            next_identifier,
        }
    }
}

#[async_trait]
impl UniqueIdentifier for EMailIdentifier {
    async fn identify(&self, identification_token: String) -> Option<String> {
        info!("Executing unique email identifier");
        if identification_token.contains('@') && identification_token.contains('.') {
            let user_id = self
                .user_repository
                .get_user_id_by_email(&identification_token)
                .await;

            match user_id {
                Ok(user_id) => {
                    return Some(user_id);
                }
                Err(err) => {
                    error!(
                        "Error getting user id with the email <{identification_token}>, error: {err}"
                    );
                }
            }
        }
        // If we couldn't handle, pass it to the next identifier, if any
        if let Some(next_identifier) = &self.next_identifier {
            next_identifier.identify(identification_token).await
        } else {
            None
        }
    }

    fn next(&self) -> Option<Arc<dyn UniqueIdentifier>> {
        self.next_identifier.clone()
    }
}

pub struct UserIdentifier {
    next_identifier: Option<Arc<dyn UniqueIdentifier>>,
}

impl UserIdentifier {
    pub fn new(next_identifier: Option<Arc<dyn UniqueIdentifier>>) -> Self {
        UserIdentifier { next_identifier }
    }
}

#[async_trait]
impl UniqueIdentifier for UserIdentifier {
    async fn identify(&self, identification_token: String) -> Option<String> {
        if Uuid::from_str(&identification_token).is_ok() {
            return Some(identification_token);
        } else {
            println!("Error identifiying token by user id, id_token: {identification_token}, the token is not UUID");
        }

        if let Some(next_identifier) = &self.next_identifier {
            next_identifier.identify(identification_token).await
        } else {
            None
        }
    }

    fn next(&self) -> Option<Arc<dyn UniqueIdentifier>> {
        self.next_identifier.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::unique_identifier_service::repository::MockUniqueIdentifierRepository;

    use super::*;
    use mockall::predicate::eq;
    use std::sync::Arc;
    use tokio;

    #[tokio::test]
    async fn test_phone_identifier_found() {
        let mut mock_repo = MockUniqueIdentifierRepository::new();

        mock_repo
            .expect_get_user_id_by_phone_number()
            .with(eq("9999999999".to_string()))
            .returning(|_| Ok("user-id-123".to_string()));

        let phone_identifier = Arc::new(PhoneIdentifier::new(Arc::new(mock_repo), None));

        let result = phone_identifier.identify("9999999999".to_string()).await;

        assert_eq!(result, Some("user-id-123".to_string()));
    }

    #[tokio::test]
    async fn test_email_identifier_found() {
        let mut mock_repo = MockUniqueIdentifierRepository::new();

        mock_repo
            .expect_get_user_id_by_email()
            .with(eq("test@example.com".to_string()))
            .returning(|_| Ok("user-id-456".to_string()));

        let email_identifier = Arc::new(EMailIdentifier::new(Arc::new(mock_repo), None));

        let result = email_identifier
            .identify("test@example.com".to_string())
            .await;

        assert_eq!(result, Some("user-id-456".to_string()));
    }

    #[tokio::test]
    async fn test_phone_identifier_invalid_number() {
        let mock_repo = MockUniqueIdentifierRepository::new();

        let phone_identifier = Arc::new(PhoneIdentifier::new(Arc::new(mock_repo), None));

        let result = phone_identifier.identify("abc123".to_string()).await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_email_identifier_invalid_email() {
        let mock_repo = MockUniqueIdentifierRepository::new();

        let email_identifier = Arc::new(EMailIdentifier::new(Arc::new(mock_repo), None));

        let result = email_identifier.identify("invalidemail".to_string()).await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_phone_identifier_with_next_email_identifier() {
        let mock_repo_phone = MockUniqueIdentifierRepository::new();

        let mut mock_repo_email = MockUniqueIdentifierRepository::new();
        mock_repo_email
            .expect_get_user_id_by_email()
            .with(eq("chained@example.com".to_string()))
            .returning(|_| Ok("chained-user-id".to_string()));

        let email_identifier = Arc::new(EMailIdentifier::new(Arc::new(mock_repo_email), None));
        let phone_identifier = Arc::new(PhoneIdentifier::new(
            Arc::new(mock_repo_phone),
            Some(email_identifier.clone()),
        ));

        let result = phone_identifier
            .identify("chained@example.com".to_string())
            .await;

        assert_eq!(result, Some("chained-user-id".to_string()));
    }
}

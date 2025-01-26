use std::sync::Arc;

use async_trait::async_trait;
use tracing::error;

use super::repository::UserRepository;

#[async_trait]
pub trait UniqueIdentifier: Sync + Send {
    async fn identify(&self, identification_token: String) -> Option<String>;

    fn next(&self) -> Option<Arc<dyn UniqueIdentifier>>;
}

pub struct PhoneIdentifier {
    user_repository: Arc<dyn UserRepository>,
    next_identifier: Option<Arc<dyn UniqueIdentifier>>,
}

impl PhoneIdentifier {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
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
    user_repository: Arc<dyn UserRepository>,
    next_identifier: Option<Arc<dyn UniqueIdentifier>>,
}

impl EMailIdentifier {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
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
    user_repository: Arc<dyn UserRepository>,
    next_identifier: Option<Arc<dyn UniqueIdentifier>>,
}

impl UserIdentifier {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        next_identifier: Option<Arc<dyn UniqueIdentifier>>,
    ) -> Self {
        UserIdentifier {
            user_repository,
            next_identifier,
        }
    }
}

#[async_trait]
impl UniqueIdentifier for UserIdentifier {
    async fn identify(&self, identification_token: String) -> Option<String> {
        if !identification_token.is_empty() {
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
                        "Error getting user id with the user id <{identification_token}>, error: {err}"
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_service::repository::MockUserRepository;
    use mockall::predicate::eq;
    use std::sync::Arc;
    use tokio;

    #[tokio::test]
    async fn test_phone_identifier_found() {
        let mut mock_repo = MockUserRepository::new();

        // Arrange: Set up the mock to return "user-id-123" when
        // get_user_id_by_phone_number is called with "9999999999"
        mock_repo
            .expect_get_user_id_by_phone_number()
            .with(eq("9999999999".to_string()))
            .returning(|_| Ok("user-id-123".to_string()));

        // Create the PhoneIdentifier with no "next" in the chain
        let phone_identifier = Arc::new(PhoneIdentifier::new(Arc::new(mock_repo), None));

        // Act
        let result = phone_identifier.identify("9999999999".to_string()).await;

        // Assert
        assert_eq!(result, Some("user-id-123".to_string()));
    }

    #[tokio::test]
    async fn test_email_identifier_found() {
        let mut mock_repo = MockUserRepository::new();

        // Arrange: Set up the mock to return "user-id-456" when
        // get_user_id_by_email is called with "test@example.com"
        mock_repo
            .expect_get_user_id_by_email()
            .with(eq("test@example.com".to_string()))
            .returning(|_| Ok("user-id-456".to_string()));

        // Create the EMailIdentifier with no "next" in the chain
        let email_identifier = Arc::new(EMailIdentifier::new(Arc::new(mock_repo), None));

        // Act
        let result = email_identifier
            .identify("test@example.com".to_string())
            .await;

        // Assert
        assert_eq!(result, Some("user-id-456".to_string()));
    }

    #[tokio::test]
    async fn test_phone_identifier_invalid_number() {
        // We can use a mock repo with no expectations for invalid input,
        // as we expect the function to simply return None early.
        let mock_repo = MockUserRepository::new();

        // Create the PhoneIdentifier with no "next" in the chain
        let phone_identifier = Arc::new(PhoneIdentifier::new(Arc::new(mock_repo), None));

        // Act: This token includes letters, so it won't pass the numeric check
        let result = phone_identifier.identify("abc123".to_string()).await;

        // Assert: We expect None because the identifier does not treat it as a valid phone number
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_email_identifier_invalid_email() {
        // Similarly, a mock repo is not needed here for any expectation
        // because we'll return None before calling the repository method.
        let mock_repo = MockUserRepository::new();

        // Create the EMailIdentifier with no "next" in the chain
        let email_identifier = Arc::new(EMailIdentifier::new(Arc::new(mock_repo), None));

        // Act: This token does not have "@" or "."
        let result = email_identifier.identify("invalidemail".to_string()).await;

        // Assert
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_phone_identifier_with_next_email_identifier() {
        // This test demonstrates chaining: the phone identifier fails first,
        // then the email identifier attempts to handle the token.

        // Mock for phone number
        let mock_repo_phone = MockUserRepository::new();
        // No expectation for phone, because we'll pass an email-like token

        // Mock for email
        let mut mock_repo_email = MockUserRepository::new();
        mock_repo_email
            .expect_get_user_id_by_email()
            .with(eq("chained@example.com".to_string()))
            .returning(|_| Ok("chained-user-id".to_string()));

        // Create the chain: EMailIdentifier is next after PhoneIdentifier
        let email_identifier = Arc::new(EMailIdentifier::new(Arc::new(mock_repo_email), None));
        let phone_identifier = Arc::new(PhoneIdentifier::new(
            Arc::new(mock_repo_phone),
            Some(email_identifier.clone()),
        ));

        // Act: Provide an email-like token that phone_identifier can't handle
        let result = phone_identifier
            .identify("chained@example.com".to_string())
            .await;

        // Assert
        assert_eq!(result, Some("chained-user-id".to_string()));
    }
}

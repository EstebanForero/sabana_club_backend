use std::sync::Arc;

use bcrypt::{hash, verify, DEFAULT_COST};

use super::domain::{UserCreationInfo, UserInfo};
use super::err::{Result, UserServiceError};
use super::repository::UserRepository;
use super::token_provider::TokenProvider;
use super::unique_identifier::UniqueIdentifier;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
    unique_identifiers: Arc<dyn UniqueIdentifier>,
    token_provider: TokenProvider,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        unique_identifiers: Arc<dyn UniqueIdentifier>,
        token_provider: TokenProvider,
    ) -> Self {
        Self {
            user_repository,
            unique_identifiers,
            token_provider,
        }
    }

    pub async fn create_user(&self, user_creation_info: UserCreationInfo) -> Result<()> {
        let hashed_password = hash(&user_creation_info.contrasena, DEFAULT_COST)
            .map_err(|err| UserServiceError::PasswordHashError(err.to_string()))?;

        let hashed_user_info = UserCreationInfo {
            contrasena: hashed_password,
            ..user_creation_info
        };

        self.user_repository.create_user(hashed_user_info).await?;
        Ok(())
    }

    pub async fn authenticate_user(&self, identifier: String, password: String) -> Result<String> {
        let user_id = match self.unique_identifiers.identify(identifier.clone()).await {
            Some(user_id) => user_id,
            None => {
                return Err(UserServiceError::AuthenticationFailed(format!(
                    "Cannot identify user with: {identifier}"
                )))
            }
        };

        let stored_password = self.user_repository.get_user_password(&user_id).await?;
        let is_authenticated = verify(&password, &stored_password)
            .map_err(|err| UserServiceError::PasswordVerificationError(err.to_string()))?;

        if !is_authenticated {
            return Err(UserServiceError::AuthenticationFailed(
                "In the password verification, the user is not authenticated".to_string(),
            ));
        }

        let token = self.token_provider.generate_token(user_id)?;
        Ok(token)
    }

    pub async fn get_users(&self) -> Result<Vec<UserInfo>> {
        Ok(self.user_repository.get_users().await?)
    }

    pub async fn get_user_by_identification(&self, identification: String) -> Result<UserInfo> {
        let user_id = self
            .unique_identifiers
            .identify(identification.clone())
            .await
            .ok_or(UserServiceError::UserNotFoundError(format!(
                "No user found for identification: {identification}"
            )))?;

        Ok(self.user_repository.get_user_by_id(&user_id).await?)
    }
}

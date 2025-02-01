use std::sync::Arc;

use bcrypt::{hash, verify, DEFAULT_COST};

use crate::unique_identifier_service::usecases::UniqueIdentifier;

use super::domain::{SearchSelection, UserCreationInfo, UserInfo, UserSelectionInfo, UserUpdating};
use super::err::{Result, UserServiceError};
use super::repository::UserRepository;
use super::token_provider::TokenProvider;

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

    pub async fn user_is_admin(&self, user_id: &str) -> Result<bool> {
        let result = self.user_repository.user_is_admin(user_id).await?;

        Ok(result)
    }

    pub async fn search_user_by_search_selection(
        &self,
        search: &str,
        limit: u8,
        search_selection: SearchSelection,
    ) -> Result<Vec<UserSelectionInfo>> {
        let users_selection_info = self
            .user_repository
            .search_users_by_search_selection(search, limit, search_selection)
            .await?;

        Ok(users_selection_info)
    }

    pub async fn update_user(&self, user_update_info: UserUpdating, user_id: &str) -> Result<()> {
        self.user_repository
            .modify_user(user_update_info, user_id)
            .await?;

        Ok(())
    }

    pub async fn create_user(&self, user_creation_info: UserCreationInfo) -> Result<()> {
        let hashed_password = hash(&user_creation_info.contrasena, DEFAULT_COST)
            .map_err(|err| UserServiceError::PasswordHashError(err.to_string()))?;

        let hashed_user_info = UserCreationInfo {
            contrasena: hashed_password,
            nombre_tipo_identificacion: user_creation_info
                .nombre_tipo_identificacion
                .to_uppercase(),
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

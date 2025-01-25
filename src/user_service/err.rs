use thiserror::Error;

use super::{repository::err::UserRepositoryError, token_provider::TokenProviderError};
use std::result;

pub type Result<T> = result::Result<T, UserServiceError>;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User repository error: {0}")]
    UserRepoError(#[from] UserRepositoryError),
    #[error("Password hashing error: {0}")]
    PasswordHashError(String),
    #[error("Password verification error: {0}")]
    PasswordVerificationError(String),
    #[error("Auth failed with message {0}")]
    AuthenticationFailed(String),
    #[error("Token provider error: {0}")]
    TokenProviderError(#[from] TokenProviderError),
}

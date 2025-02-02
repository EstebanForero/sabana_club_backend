use thiserror::Error;

use crate::user_service::err::UserServiceError;

use super::repository::err::RequestRepositoryError;

pub type Result<T> = std::result::Result<T, RequestServiceError>;

#[derive(Debug, Error)]
pub enum RequestServiceError {
    #[error("Error executing a command in the user service: {0}")]
    CommandExecutionError(#[from] UserServiceError),
    #[error("request repository error: {0}")]
    RequestRepositoryError(#[from] RequestRepositoryError),
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

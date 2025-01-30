use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, UserRepositoryError>;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("Internal database error: {0}")]
    InternalDbError(#[from] Box<dyn std::error::Error>),
    #[error("User not found")]
    UserNotFound,
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] libsql::Error),
    #[error("Error deserializing into a struct form the database: {0}")]
    DeserializationError(#[from] serde::de::value::Error),
    #[error("chrono date operation error: {0}")]
    DateOperationError(#[from] chrono::ParseError),
}

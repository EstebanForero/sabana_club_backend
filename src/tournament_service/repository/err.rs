use std::error::Error;

pub type Result<T> = std::result::Result<T, TournamentRepositoryError>;

#[derive(thiserror::Error, Debug)]
pub enum TournamentRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Tournament not found")]
    TournamentNotFound,
    #[error("User already registered in tournament")]
    UserAlreadyRegistered,
}

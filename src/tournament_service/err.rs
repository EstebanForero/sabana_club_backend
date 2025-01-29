use super::repository::err::TournamentRepositoryError;

pub type Result<T> = std::result::Result<T, TournamentServiceError>;

#[derive(thiserror::Error, Debug)]
pub enum TournamentServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] TournamentRepositoryError),
    #[error("Tournament not found")]
    TournamentNotFound,
    #[error("User already registered in tournament")]
    UserAlreadyRegistered,
    #[error("Could not identify user with identificator: {0}")]
    UserNotIdentifiable(String),
}

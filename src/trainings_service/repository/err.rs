pub type Result<T> = std::result::Result<T, TrainingRepositoryError>;

#[derive(thiserror::Error, Debug)]
pub enum TrainingRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Training not found")]
    TrainingNotFound,
    #[error("User already registered in training")]
    UserAlreadyRegistered,
}

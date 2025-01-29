use super::repository::err::TrainingRepositoryError;

pub type Result<T> = std::result::Result<T, TrainingServiceError>;

#[derive(thiserror::Error, Debug)]
pub enum TrainingServiceError {
    #[error("Training repository error: {0}")]
    TrainingRepositoryError(#[from] TrainingRepositoryError),
    #[error("Could not identify user with identificator: {0}")]
    UserNotIdentifiable(String),
}

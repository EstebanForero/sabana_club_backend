use thiserror::Error;

use super::repository::err::TuitionRepositoryError;

pub type Result<T> = std::result::Result<T, TuitionServiceError>;

#[derive(Error, Debug)]
pub enum TuitionServiceError {
    #[error("Error in the tuition repository: {0}")]
    TuitionRepositoryError(#[from] TuitionRepositoryError),
}

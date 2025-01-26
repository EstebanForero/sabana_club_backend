pub type Result<T> = std::result::Result<T, TuitionRepositoryError>;

#[derive(thiserror::Error, Debug)]
pub enum TuitionRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Tuition not found")]
    TuitionNotFound,
}

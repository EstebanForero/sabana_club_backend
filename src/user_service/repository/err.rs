use std::result;

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
enum Error {
    #[error("Internal database error: {0}")]
    InternalDbError(#[from] Box<dyn std::error::Error>),
}

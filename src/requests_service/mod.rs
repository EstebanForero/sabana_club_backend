use serde::{Deserialize, Serialize};

pub mod domain;
mod err;
pub mod repository;
pub mod usecases;

#[derive(Serialize, Deserialize, Debug)]
enum Commands {
    CreateUser(),
}

use crate::trainings_service::model::{Training, TrainingRegistration};
use async_trait::async_trait;
use mockall::automock;

pub mod err;
use err::Result;
pub mod lib_sql_implementation;

#[automock]
#[async_trait]
pub trait TrainingRepository: Send + Sync {
    /// Creates a new training session.
    async fn create_training(&self, training: Training) -> Result<()>;

    /// Registers a user in a training session.
    async fn register_user_in_training(&self, registration: TrainingRegistration) -> Result<()>;

    /// Retrieves all training sessions.
    async fn get_all_trainings(&self) -> Result<Vec<Training>>;

    /// Retrieves all users registered in a specific training session.
    async fn get_users_in_training(
        &self,
        id_entrenamiento: &str,
    ) -> Result<Vec<TrainingRegistration>>;

    async fn get_trainings_for_user(&self, user_id: &str) -> Result<Vec<Training>>;

    async fn delete_training(&self, training_id: &str) -> Result<()>;
}

use std::sync::Arc;

use uuid::Uuid;

use crate::unique_identifier_service::usecases::UniqueIdentifier;

use super::err::{Result, TrainingServiceError};

use super::model::{Training, TrainingRegistration};
use super::repository::TrainingRepository;

#[derive(Clone)]
pub struct TrainingService {
    training_repository: Arc<dyn TrainingRepository>,
    unique_identifier: Arc<dyn UniqueIdentifier>,
}

impl TrainingService {
    pub fn new(
        training_repository: Arc<dyn TrainingRepository>,
        unique_identifier: Arc<dyn UniqueIdentifier>,
    ) -> Self {
        Self {
            training_repository,
            unique_identifier,
        }
    }

    pub async fn get_training(&self, training_id: &str) -> Result<Training> {
        let training = self.training_repository.get_training(training_id).await?;

        Ok(training)
    }

    pub async fn delete_training(&self, training_id: &str) -> Result<()> {
        self.training_repository
            .delete_training(training_id)
            .await?;

        Ok(())
    }

    pub async fn get_trainings_for_user(
        &self,
        user_identification: String,
    ) -> Result<Vec<Training>> {
        let user_id = self
            .unique_identifier
            .identify(user_identification.clone())
            .await;

        let user_id = match user_id {
            Some(user_id) => user_id,
            None => {
                return Err(TrainingServiceError::UserNotIdentifiable(
                    user_identification,
                ))
            }
        };

        let user_trainings = self
            .training_repository
            .get_trainings_for_user(&user_id)
            .await?;

        Ok(user_trainings)
    }

    pub async fn create_training(
        &self,
        nombre_entrenamiento: String,
        tiempo_minutos: i32,
    ) -> Result<String> {
        let id_entrenamiento = Uuid::new_v4().to_string();
        let training = Training {
            id_entrenamiento: id_entrenamiento.clone(),
            tiempo_minutos,
            nombre_entrenamiento,
        };

        self.training_repository.create_training(training).await?;
        Ok(id_entrenamiento)
    }

    pub async fn register_user_in_training(
        &self,
        id_entrenamiento: String,
        id_persona: String,
    ) -> Result<()> {
        let registration = TrainingRegistration {
            id_entrenamiento,
            id_persona,
        };

        self.training_repository
            .register_user_in_training(registration)
            .await?;

        Ok(())
    }

    pub async fn get_all_trainings(&self) -> Result<Vec<Training>> {
        Ok(self.training_repository.get_all_trainings().await?)
    }

    pub async fn get_users_in_training(
        &self,
        id_entrenamiento: String,
    ) -> Result<Vec<TrainingRegistration>> {
        Ok(self
            .training_repository
            .get_users_in_training(&id_entrenamiento)
            .await?)
    }
}

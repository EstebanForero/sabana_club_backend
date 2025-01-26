use std::sync::Arc;

use uuid::Uuid;

use super::err::Result;

use super::model::{Training, TrainingRegistration};
use super::repository::TrainingRepository;

#[derive(Clone)]
pub struct TrainingService {
    training_repository: Arc<dyn TrainingRepository>,
}

impl TrainingService {
    pub fn new(training_repository: Arc<dyn TrainingRepository>) -> Self {
        Self {
            training_repository,
        }
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

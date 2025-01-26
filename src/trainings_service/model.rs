use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Training {
    pub id_entrenamiento: String,
    pub nombre_entrenamiento: String,
    pub tiempo_minutos: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRegistration {
    pub id_entrenamiento: String,
    pub id_persona: String,
}

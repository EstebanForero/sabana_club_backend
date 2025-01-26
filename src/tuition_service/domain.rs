use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tuition {
    pub id_persona: String,
    pub monto_usd: f64,
    pub fecha_inscripccion: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TuitionInfo {
    pub id_persona: String,
    pub monto_usd: f64,
}

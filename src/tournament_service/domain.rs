use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub id_torneo: String,
    pub nombre: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTournamentRegistration {
    pub id_persona: String,
    pub id_torneo: String,
    pub puesto: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTournamentInfo {
    pub id_torneo: String,
    pub nombre: String,
    pub puesto: i32,
}

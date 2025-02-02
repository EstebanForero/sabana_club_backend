use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserCreationInfo {
    pub nombre: String,
    pub contrasena: String,
    pub correo: String,
    pub telefono: u64,
    pub identificacion: String,
    pub nombre_tipo_identificacion: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserUpdating {
    pub nombre: String,
    pub correo: String,
    pub telefono: u64,
    pub identificacion: String,
    pub nombre_tipo_identificacion: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub id_persona: String,
    pub nombre: String,
    pub correo: String,
    pub telefono: u64,
    pub identificacion: String,
    pub nombre_tipo_identificacion: String,
    pub es_admin: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UserSelectionInfo {
    pub id_persona: String,
    pub nombre: String,
    pub correo: String,
    pub telefono: u64,
    pub identificacion: String,
    pub nombre_tipo_identificacion: String,
    pub es_admin: bool,
    pub matricula_valida: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SearchSelection {
    Email,
    PhoneNumber,
    UserName,
}

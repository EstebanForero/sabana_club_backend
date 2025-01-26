use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserCreationInfo {
    pub nombre: String,
    pub contrasena: String,
    pub correo: String,
    pub telefono: String,
    pub identificacion: String,
    pub nombre_tipo_identificacion: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub id_persona: String,
    pub nombre: String,
    pub correo: String,
    pub telefono: String,
    pub identificacion: String,
    pub nombre_tipo_identificacion: String,
    pub es_admin: bool,
    pub fecha_ingreso: String,
}

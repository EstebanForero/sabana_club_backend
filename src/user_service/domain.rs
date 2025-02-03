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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
    pub nombre_rol: UserRol,
}

#[derive(Deserialize, Serialize)]
pub struct UserSelectionInfo {
    pub id_persona: String,
    pub nombre: String,
    pub correo: String,
    pub telefono: u64,
    pub identificacion: String,
    pub nombre_tipo_identificacion: String,
    pub nombre_rol: UserRol,
    pub matricula_valida: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UserRol {
    Usuario,
    Admin,
    Entrenador,
}

impl ToString for UserRol {
    fn to_string(&self) -> String {
        match self {
            UserRol::Usuario => "Usuario",
            UserRol::Admin => "Admin",
            UserRol::Entrenador => "Entrenador",
        }
        .to_string()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SearchSelection {
    Email,
    PhoneNumber,
    UserName,
}

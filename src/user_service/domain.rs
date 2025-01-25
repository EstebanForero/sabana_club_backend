use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserCreationInfo {
    pub nombre: String,
    pub contrasena: String,
    pub correo: String,
    pub telefono: String,
    pub identificacion: String,
    pub nombre_tipo_identificacion: String,
}

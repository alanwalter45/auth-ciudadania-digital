use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseInformation {
    sub: String,
    profile: Profile,
    email: String,
    celular: String,
    fecha_nacimiento: String,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    documento_identidad: Document,
    nombre: Name,
}

#[derive(Serialize, Deserialize)]
pub struct Document {
    numero_documento: String,
    tipo_documento: String,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    nombres: String,
    primer_apellido: String,
    segundo_apellido: String,
}

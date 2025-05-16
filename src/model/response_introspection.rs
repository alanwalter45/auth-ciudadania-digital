use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseIntrospection {
    pub active: bool,
    pub sub: String,
    pub client_id: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub scope: String,
    pub token_type: String,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseToken {
    pub access_token: String,
    pub expires_in: i32,
    pub id_token: String,
    pub scope: String,
    pub token_type: String,
    pub refresh_token: String,
}

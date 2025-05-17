use std::sync::Mutex;

pub struct AppState {
    pub url: String,
    pub provider_url: String,
    pub client_id: String,
    pub nonce: String,
    pub state: String,
    pub secret: String,
    pub access_token: Mutex<String>,
    pub id_token: Mutex<String>,
}

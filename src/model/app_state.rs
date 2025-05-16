use std::sync::Mutex;

pub struct AppState {
    pub url: String,
    pub provider_url: String,
    pub client_id: String,
    pub nonce: String,
    pub secret: String,
    pub redirect_uri: String,
    pub access_token: Mutex<String>,
    pub id_token: Mutex<String>,
}

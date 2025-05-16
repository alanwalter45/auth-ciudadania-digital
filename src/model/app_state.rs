use std::sync::Mutex;

pub struct AppState {
    pub url: String,
    pub provider_url: String,
    pub access_token: Mutex<String>,
    pub id_token: Mutex<String>,
}

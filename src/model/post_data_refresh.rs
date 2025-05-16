#[derive(Debug, serde::Serialize)]
pub struct PostDataRefresh {
    pub grant_type: String,
    pub refresh_token: String,
    //pub client_id: String,
    //pub client_secret: String,
}

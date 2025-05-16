#[derive(Debug, serde::Serialize)]
pub struct PostData {
    pub code: String,
    pub redirect_uri: String,
    pub grant_type: String,
    //pub code_verifier: String,
    //pub client_id: String,
    //pub client_secret: String,
}

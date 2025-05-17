#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ParamAuthentication {
    pub redirect_uri: Option<String>,
}

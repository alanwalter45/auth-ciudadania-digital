#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ParamAuthorization {
    pub code: Option<String>,
    pub state: Option<String>,
}

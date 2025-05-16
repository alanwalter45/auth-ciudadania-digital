#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ParamRefresh {
    pub refresh_token: String,
}

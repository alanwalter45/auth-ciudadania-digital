#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ParamLogout {
    pub id_token_hint: Option<String>,
    pub post_logout_redirect_uri: Option<String>,
}

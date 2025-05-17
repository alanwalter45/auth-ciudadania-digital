#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ParamVerify {
    pub application_name: Option<String>,
}

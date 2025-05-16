#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ParamAuthorization {
    pub code: String,
}

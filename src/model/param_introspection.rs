#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ParamIntrospection {
    pub token: String,
}

use validator::Validate;

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
pub struct ParamRefresh {
    #[validate(
        required,
        length(min = 10, message = "refeshtoken must be greater than 10 chars")
    )]
    pub refresh_token: Option<String>,
}

use validator::Validate;

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
pub struct ParamAuthorization {
    #[validate(
        required,
        length(min = 15, message = "code must be greater than 15 chars")
    )]
    pub code: Option<String>,
    #[validate(
        required,
        length(min = 30, message = "state must be greater than 30 chars")
    )]
    pub state: Option<String>,
}

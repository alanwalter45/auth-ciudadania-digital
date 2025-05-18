use validator::Validate;

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
pub struct ParamAuthentication {
    #[validate(
        required,
        length(min = 15, message = "redirect must be greater than 15 chars")
    )]
    pub redirect_uri: Option<String>,
}

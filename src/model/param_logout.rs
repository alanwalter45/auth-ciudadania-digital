use validator::Validate;

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
pub struct ParamLogout {
    #[validate(
        required,
        length(min = 10, message = "id_token_hint must be greater than 10 chars")
    )]
    pub id_token_hint: Option<String>,
    #[validate(
        required,
        length(min = 15, message = "post_logout_redirect_uri must be greater than 15 chars")
    )]
    pub post_logout_redirect_uri: Option<String>,
}

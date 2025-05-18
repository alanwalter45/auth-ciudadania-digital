use validator::Validate;

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
pub struct ParamIntrospection {
    #[validate(
        required,
        length(min = 10, message = "token must be greater than 10 chars")
    )]
    pub token: Option<String>,
}

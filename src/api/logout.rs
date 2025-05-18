use crate::AppState;
use actix_web::{HttpResponse, Responder, get, web};
use validator::Validate;

#[utoipa::path(
    post,
    tag = "API",
    path = "/logout",
    responses(
        (status = 200, description = "Get url to logout"),
        (status = 400, description = "redirect_uri is requires")
    ),
    params(
        ("json"=ParamLogout,description="data required")
    ),
    description = "Generate URL for logout"
)]
#[get("/logout")]
pub async fn logout(data: web::Data<AppState>, json: web::Json<ParamLogout>) -> impl Responder {
    match json.validate() {
        Ok(_) => {
            let url = format!(
                "{}/session/end?id_token_hint={}&post_logout_redirect_uri={}",
                data.provider_url,
                json.id_token_hint.as_deref().unwrap(),
                json.post_logout_redirect_uri.as_deref().unwrap()
            );
            HttpResponse::Ok().body(url)
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
pub struct ParamLogout {
    #[validate(
        required,
        length(min = 10, message = "id_token_hint must be greater than 10 chars")
    )]
    id_token_hint: Option<String>,
    #[validate(
        required,
        length(min = 15, message = "post_logout_redirect_uri must be greater than 15 chars")
    )]
    post_logout_redirect_uri: Option<String>,
}
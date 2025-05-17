use crate::model::{app_state::*, param_logout::*};
use actix_web::{HttpResponse, Responder, get, web};

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
    if let (Some(id_token_hint), Some(post_logout_redirect_uri)) =
        (json.post_logout_redirect_uri.clone(), json.id_token_hint.clone())
    {
        let url = format!(
            "{}/session/end?id_token_hint={}&post_logout_redirect_uri={}",
            data.provider_url, id_token_hint, post_logout_redirect_uri
        );
        HttpResponse::Ok().body(url)
    } else {
        HttpResponse::BadRequest().body("Arguments required not found.")
    }
}

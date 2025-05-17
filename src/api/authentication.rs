use crate::model::{app_state::*, param_authentication::ParamAuthentication};
use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    tag = "API",
    path = "/authentication",
    responses(
        (status = 200, description = "Get url to redirect authentication"),
        (status = 400, description = "redirect_uri is requires")
    ),
    params(
        ("redirect_uri"=String,Query,description="URI to redirect after login")
    )
)]
#[get("/authentication")]
pub async fn authentication(
    data: web::Data<AppState>,
    params: web::Query<ParamAuthentication>,
) -> impl Responder {
    if let Some(redirect_uri) = &params.redirect_uri {
        let url = format!(
            "{}/auth?response_type=code&client_id={}&state={}&nonce={}&redirect_uri={}&scope=openid%20profile%20fecha_nacimiento%20email%20celular%20offline_access&prompt=consent",
            data.provider_url, data.client_id, data.state, data.nonce, redirect_uri,
        );
        HttpResponse::Ok().body(url)
    } else {
        HttpResponse::BadRequest().body("Argument required not found.")
    }
}

use crate::model::app_state::*;
use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    tag = "API",
    path = "/authentication",
    responses(
        (status = 200, description = "Get url to redirect authentication")
    ),
    params(
        ("redirect_uri"=String, description="url to go after authentication"),
    )
)]
#[get("/authentication")]
pub async fn authentication(data: web::Data<AppState>) -> impl Responder {
    let url = format!(
        "{}/auth?response_type=code&client_id={}&nonce={}&redirect_uri={}&scope=openid%20profile%20fecha_nacimiento%20email%20celular%20offline_access&prompt=consent",
        data.provider_url, data.client_id, data.nonce, data.redirect_uri,
    );
    HttpResponse::Ok().body(url)
}

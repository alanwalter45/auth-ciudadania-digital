use crate::AppState;
use actix_web::{HttpResponse, Responder, get, web};
use validator::Validate;

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
    ),
    description = "Generate URL for authenticate via AGETIC"
)]
#[get("/authentication")]
pub async fn authentication(
    data: web::Data<AppState>,
    params: web::Query<ParamAuthentication>,
) -> impl Responder {
    match params.validate() {
        Ok(_) => {
            let url = format!(
                "{}/auth?response_type=code&client_id={}&state={}&nonce={}&redirect_uri={}&scope=openid%20profile%20fecha_nacimiento%20email%20celular%20offline_access&prompt=consent",
                data.provider_url,
                data.client_id,
                data.state,
                data.nonce,
                params.redirect_uri.clone().unwrap(),
            );
            HttpResponse::Ok().body(url)
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
struct ParamAuthentication {
    #[validate(
        required,
        length(min = 15, message = "redirect must be greater than 15 chars")
    )]
    redirect_uri: Option<String>,
}

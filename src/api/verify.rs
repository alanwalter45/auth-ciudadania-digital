use crate::model::param_verify::*;
use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    tag = "API",
    path = "/verify",
    responses(
        (status = 200, description = "Get url to redirect authentication"),
    ),
    params(
        ("application_name"=String,Query,description="URI to redirect after login")
    ),
    description = "Get URL for authenticate via AGETIC"
)]
#[get("/verify")]
pub async fn verify(params: web::Query<ParamVerify>) -> impl Responder {
    if let Some(application_name) = &params.application_name {
        HttpResponse::Ok().body(application_name.clone())
    } else {
        HttpResponse::BadRequest().body("Argument required not found.")
    }
}

use crate::AppState;

use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    tag = "API",
    path = "/",
    responses(
        (status = 200, description = "Redirect to authentication")
    )
)]
#[get("/")]
pub async fn login(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Found()
        .insert_header(("Location", format!("{}", data.url_client)))
        .finish()
}

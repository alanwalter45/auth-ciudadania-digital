use actix_web::{HttpResponse, Responder, get, web};

use crate::model::app_state::*;

#[get("/get-info")]
pub async fn get_info(data: web::Data<AppState>) -> impl Responder {
    let body = data.access_token.lock().unwrap();
    HttpResponse::Ok().json(body.to_string())
}

use crate::{AppState, api::authorization::ResponseError, api::introspection::*};
use actix_web::{HttpResponse, Responder, post, web};
use awc::{ClientBuilder, Connector};
use validator::Validate;

#[utoipa::path(
    post,
    tag = "API",
    path = "/information",
    responses(
        (status = 200, description = "Get information of user")
    ),
    params(
        ("token"=ParamIntrospection, description="token get on authorization"),
    )
)]
#[post("/information")]
pub async fn information(
    data: web::Data<AppState>,
    json: web::Json<ParamIntrospection>,
) -> impl Responder {
    match json.validate() {
        Ok(_) => {
            let token = json.token.as_deref().unwrap();
            let url = format!("{}/me", data.provider_url);
            let client = ClientBuilder::new().connector(Connector::new()).finish();
            let mut response = client
                .get(url)
                .insert_header(("authorization", format!("Bearer {}", token)))
                .send()
                .await
                .unwrap();

            let response_information: ExternalAPiResponse = response.json().await.unwrap();
            let output = match response_information {
                ExternalAPiResponse::Success(data) => HttpResponse::Ok().json(&data),
                ExternalAPiResponse::Error(err) => HttpResponse::BadRequest().json(&err),
            };

            if response.status().is_success() {
                output
            } else {
                HttpResponse::InternalServerError().body("GET request failed.")
            }
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResponseInformation {
    sub: String,
    profile: Profile,
    email: String,
    celular: String,
    fecha_nacimiento: String,
}

#[derive(Serialize, Deserialize)]
struct Profile {
    documento_identidad: Document,
    nombre: Name,
}

#[derive(Serialize, Deserialize)]
struct Document {
    numero_documento: String,
    tipo_documento: String,
}

#[derive(Serialize, Deserialize)]
struct Name {
    nombres: String,
    primer_apellido: String,
    segundo_apellido: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum ExternalAPiResponse {
    Success(ResponseInformation),
    Error(ResponseError),
}

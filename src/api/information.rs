use crate::model::app_state::*;
use crate::model::param_introspection::*;
use crate::model::response_information::*;
use actix_web::{HttpResponse, Responder, post, web};
use awc::{ClientBuilder, Connector};

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
    let token = json.token.clone();
    let url = format!("{}/me", data.provider_url);
    let client = ClientBuilder::new().connector(Connector::new()).finish();
    let mut response = client
        .get(url)
        .insert_header(("authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();

    let response_introspection: ResponseInformation = response.json().await.unwrap();

    if response.status().is_success() {
        HttpResponse::Ok().json(&response_introspection)
    } else {
        HttpResponse::InternalServerError().body("GET request failed.")
    }
}

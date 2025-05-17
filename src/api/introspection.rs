use super::resources::credential::*;
use crate::model::app_state::*;
use crate::model::param_introspection::*;
use crate::model::post_data_introspection::*;
use crate::model::response_introspection::*;
use actix_web::{HttpResponse, Responder, post, web};
use awc::{ClientBuilder, Connector};

#[utoipa::path(
    post,
    tag = "API",
    path = "/introspection",
    responses(
        (status = 200, description = "Get token via authentication")
    ),
    params(
        ("token"=ParamIntrospection, description="token get on authorization"),
    )
)]
#[post("/introspection")]
pub async fn introspection(
    data: web::Data<AppState>,
    json: web::Json<ParamIntrospection>,
) -> impl Responder {
    let token = json.token.clone();
    let url = format!("{}/token/introspection", data.provider_url);
    let post_data = PostDataIntrospection { token };
    let credential = get_credential(data.client_id.clone(), data.secret.clone());
    let client = ClientBuilder::new().connector(Connector::new()).finish();
    let mut response = client
        .post(url)
        .insert_header(("cache-control", "no-cache"))
        .insert_header(("authorization", credential))
        .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
        .send_body(serde_urlencoded::to_string(&post_data).unwrap())
        .await
        .unwrap();

    let response_introspection: ResponseIntrospection = response.json().await.unwrap();

    if response.status().is_success() {
        HttpResponse::Ok().json(&response_introspection)
    } else {
        HttpResponse::InternalServerError().body("POST request failed.")
    }
}

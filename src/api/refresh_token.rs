use crate::api::resources::credential::get_credential;
use crate::model::app_state::*;
use crate::model::param_refresh::*;
use crate::model::post_data_refresh::*;
use crate::model::response_token::*;
use actix_web::{HttpResponse, Responder, post, web};
use awc::{ClientBuilder, Connector};

#[post("/refresh-token")]
pub async fn refresh_token(
    json: web::Json<ParamRefresh>,
    data: web::Data<AppState>,
) -> impl Responder {
    let refresh_token = json.refresh_token.clone();
    let url = format!("{}{}", data.provider_url, "/token");
    let post_data = PostDataRefresh {
        grant_type: "refresh_token".to_string(),
        refresh_token,
    };

    let credential = get_credential();

    let client = ClientBuilder::new().connector(Connector::new()).finish();
    let mut response = client
        .post(url)
        .insert_header(("cache-control", "no-cache"))
        .insert_header(("authorization", credential))
        .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
        .send_body(serde_urlencoded::to_string(&post_data).unwrap())
        .await
        .unwrap();

    let token: ResponseToken = response.json().await.unwrap();

    let mut access_token = data.access_token.lock().unwrap();
    *access_token = token.access_token.clone();
    let mut id_token = data.id_token.lock().unwrap();
    *id_token = token.id_token.clone();

    if response.status().is_success() {
        HttpResponse::Ok().json(&token)
    } else {
        HttpResponse::InternalServerError().body("POST request failed.")
    }
}

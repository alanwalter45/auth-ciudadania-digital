use super::resources::credential::get_credential;
use crate::model::{app_state::*, param_authorization::*, post_data::PostData, response_token::*};
use actix_web::{HttpResponse, Responder, post, web};
use awc::{ClientBuilder, Connector};

#[utoipa::path(
    post,
    tag = "api",
    path = "/authorization",
    responses(
        (status = 200, description = "Get token via authentication")
    ),
    params(
        ("code"=String, description="code request"),
    )
)]
#[post("/authorization")]
pub async fn authorization(
    json: web::Json<ParamAuthorization>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = json.code.clone();
    let url = format!("{}{}", data.provider_url, "/token");
    let post_data = PostData {
        code,
        redirect_uri: format!("{}{}", data.url, "/autenticacion"),
        grant_type: "authorization_code".to_string(),
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

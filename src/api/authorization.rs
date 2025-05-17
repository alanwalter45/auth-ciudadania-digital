use super::resources::credential::get_credential;
use crate::model::{app_state::*, param_authorization::*, post_data::*, response_token::*};
use actix_web::{HttpResponse, Responder, get, web};
use awc::{ClientBuilder, Connector};

#[utoipa::path(
    get,
    tag = "API",
    path = "/authorization",
    responses(
        (status = 200, description = "Get token via authentication"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("code"=String,Query, description="code after authentication"),
        ("state"=String,Query, description="field state official"),
    ),
    description="Generate Token"
)]
#[get("/authorization")]
pub async fn authorization(
    data: web::Data<AppState>,
    params: web::Query<ParamAuthorization>,
) -> impl Responder {
    if let (Some(code), Some(state)) = (params.code.clone(), params.state.clone()) {
        if state == data.state {
            let post_data = PostData {
                code,
                redirect_uri: format!("{}/autenticacion", data.url),
                grant_type: "authorization_code".to_string(),
            };

            let url = format!("{}/token", data.provider_url);
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

            let token: ResponseToken = response.json().await.unwrap();

            if response.status().is_success() {
                HttpResponse::Ok().json(&token)
            } else {
                HttpResponse::InternalServerError().body("POST request failed.")
            }
        } else {
            HttpResponse::BadRequest().body("State not matching.")
        }
    } else {
        HttpResponse::BadRequest().body("Arguments required not found.")
    }
}

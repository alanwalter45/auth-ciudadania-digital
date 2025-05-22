use crate::AppState;
use crate::api::authorization::ExternalAPiResponse;
use crate::resources::credential::*;
use actix_web::{HttpResponse, Responder, post, web};
use awc::{ClientBuilder, Connector};
use validator::Validate;

#[utoipa::path(
    post,
    tag = "API",
    path = "/refresh-token",
    responses(
        (status = 200, description = "Get field refresh_token via authentication")
    ),
    params(
        ("json"=ParamRefresh, description="refresh_token get on authorization"),
    )
)]
#[post("/refresh-token")]
pub async fn refreshtoken(
    data: web::Data<AppState>,
    json: web::Json<ParamRefresh>,
) -> impl Responder {
    match json.validate() {
        Ok(_) => {
            let url = format!("{}/token", data.provider_url);
            let post_data = PostDataRefresh {
                grant_type: "refresh_token".to_string(),
                refresh_token: json.refresh_token.clone().unwrap(),
            };

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

            let token: ExternalAPiResponse = response.json().await.unwrap();

            let output = match token {
                ExternalAPiResponse::Success(data) => HttpResponse::Ok().json(&data),
                ExternalAPiResponse::Error(err) => HttpResponse::BadRequest().json(&err),
            };

            if response.status().is_success() {
                output
            } else {
                HttpResponse::InternalServerError().body("POST request failed.")
            }
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
pub struct ParamRefresh {
    #[validate(
        required,
        length(min = 10, message = "refeshtoken must be greater than 10 chars")
    )]
    refresh_token: Option<String>,
}

#[derive(serde::Serialize)]
struct PostDataRefresh {
    grant_type: String,
    refresh_token: String,
    //client_id: String,
    //client_secret: String,
}

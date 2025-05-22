use crate::api::authorization::ResponseError;
use crate::AppState;
use crate::resources::credential::*;
use actix_web::{HttpResponse, Responder, post, web};
use awc::{ClientBuilder, Connector};
use validator::Validate;

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
    match json.validate() {
        Ok(_) => {
            let token = json.token.clone().unwrap();
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

            let response_introspection: ExternalAPiResponse = response.json().await.unwrap();
            let output = match response_introspection {
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
pub struct ParamIntrospection {
    #[validate(
        required,
        length(min = 10, message = "token must be greater than 10 chars")
    )]
    pub token: Option<String>,
}

#[derive(serde::Serialize)]
struct PostDataIntrospection {
    token: String,
    //client_id: String,
    //client_secret: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ResponseIntrospection {
    active: bool,
    sub: String,
    client_id: String,
    exp: i64,
    iat: i64,
    iss: String,
    scope: String,
    token_type: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum ExternalAPiResponse {
    Success(ResponseIntrospection),
    Error(ResponseError),
}

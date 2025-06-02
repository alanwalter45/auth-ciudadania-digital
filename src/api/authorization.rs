use crate::AppState;
use crate::resources::credential::get_credential;
use actix_web::{HttpResponse, Responder, get, web};
use awc::{ClientBuilder, Connector};
use validator::Validate;

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
        ("redirect_uri"=String,Query, description="field redirect_uri to use after login"),
    ),
    description="Get Token AGETIC"
)]
#[get("/authorization")]
pub async fn authorization(
    data: web::Data<AppState>,
    params: web::Query<ParamAuthorization>,
) -> impl Responder {
    match params.validate() {
        Ok(_) => {
            if params.state.clone().unwrap() == data.state {
                let post_data = PostData {
                    code: params.code.clone().unwrap(),
                    redirect_uri: params.redirect_uri.clone().unwrap(),
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

                let token: ExternalAPiResponse = response.json().await.unwrap();

                let output = match token {
                    ExternalAPiResponse::Success(data) => HttpResponse::Ok().json(&data),
                    ExternalAPiResponse::Error(err) => HttpResponse::BadRequest().json(&err),
                };
                if response.status().is_success() {
                    output
                } else {
                    HttpResponse::BadRequest().body("POST request failed.")
                }
            } else {
                HttpResponse::BadRequest().body("State not matching.")
            }
        }
        Err(_) => HttpResponse::BadRequest().json("Arguments required not found."),
    }
}

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
struct ParamAuthorization {
    #[validate(
        required,
        length(min = 15, message = "code must be greater than 15 chars")
    )]
    code: Option<String>,
    #[validate(
        required,
        length(min = 15, message = "state must be greater than 30 chars")
    )]
    state: Option<String>,
    #[validate(
        required,
        length(min = 5, message = "redirect URI must be greater than 5 chars")
    )]
    redirect_uri: Option<String>
}

#[derive(serde::Serialize)]
struct PostData {
    code: String,
    redirect_uri: String,
    grant_type: String,
    //code_verifier: String,
    //client_id: String,
    //client_secret: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseToken {
    pub access_token: String,
    pub expires_in: i32,
    pub id_token: String,
    pub scope: String,
    pub token_type: String,
    pub refresh_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseError {
    pub error: String,
    pub message: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ExternalAPiResponse {
    Success(ResponseToken),
    Error(ResponseError),
}

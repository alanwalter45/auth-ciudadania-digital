use actix_cors::Cors;
use actix_web::{
    App, HttpServer, http,
    web::{self},
};
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod api;
use api::{
    authentication::*, authorization::*, information::*, introspection::*, logout::*,
    refresh_token::*,
};
mod resources;

#[derive(OpenApi)]
#[openapi(
        paths(authentication,authorization,information, introspection, refreshtoken,logout),
        components(
            schemas(ParamIntrospection, ParamRefresh,ParamLogout)
        ),
        tags(
            (name = "API", description = "Management endpoints.")
        )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_state = web::Data::new(AppState {
        url_server: env::var("APP_URL_SERVER").unwrap(),
        provider_url: env::var("APP_URL_PROVIDER_AGETIC").unwrap(),
        client_id: env::var("APP_CLIENT_ID").unwrap(),
        secret: env::var("APP_SECRET").unwrap(),
        state: env::var("APP_NONCE").unwrap(),
        nonce: env::var("APP_STATE").unwrap(),
    });
    let ip = env::var("APP_IP").unwrap();
    let port = env::var("APP_PORT").unwrap();
    let port = port.parse().expect("Port is Not a Number");
    let allow_urls = env::var("APP_ALLOW_URLS").unwrap();
    let allow_urls: Vec<String> = allow_urls.split(' ').map(|s| s.to_string()).collect();

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        for origin in &allow_urls {
            cors = cors.allowed_origin(origin);
        }
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(authentication)
            .service(authorization)
            .service(information)
            .service(refreshtoken)
            .service(introspection)
            .service(logout)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((ip, port))?
    .run()
    .await
}

pub struct AppState {
    pub url_server: String,
    pub provider_url: String,
    pub client_id: String,
    pub nonce: String,
    pub state: String,
    pub secret: String,
}

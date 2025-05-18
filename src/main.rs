use actix_web::{
    App, HttpServer,
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
        url: env::var("APP_URL").unwrap(),
        provider_url: env::var("APP_URL_PROVIDER_AGETIC").unwrap(),
        client_id: env::var("APP_CLIENT_ID").unwrap(),
        secret: env::var("APP_SECRET").unwrap(),
        state: env::var("APP_NONCE").unwrap(),
        nonce: env::var("APP_STATE").unwrap(),
    });
    let ip = env::var("APP_IP").unwrap();
    let port = env::var("APP_PORT").unwrap();
    let port = port.parse().expect("Port is Not a Number");

    HttpServer::new(move || {
        App::new()
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
    pub url: String,
    pub provider_url: String,
    pub client_id: String,
    pub nonce: String,
    pub state: String,
    pub secret: String,
}

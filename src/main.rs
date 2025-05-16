use actix_web::{
    App, HttpServer,
    web::{self},
};
use dotenv::dotenv;
use std::{env, sync::Mutex};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod api;
use api::{authorization::*, refresh_token::*};
use api::introspection::*;
use api::get_info::*;
mod model;
use model::{app_state, param_authorization::*};

#[derive(OpenApi)]
#[openapi(
        paths(authorization),
        components(
            schemas(ParamAuthorization)
        ),
        tags(
            (name = "api", description = "Management endpoints.")
        ),
    )]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_state = web::Data::new(app_state::AppState {
        url: env::var("APP_URL").unwrap(),
        provider_url: env::var("APP_URL_PROVIDER_AGETIC").unwrap(),
        access_token: Mutex::new("".to_string()),
        id_token: Mutex::new("".to_string()),
    });
    let ip = env::var("APP_IP").unwrap();
    let port = env::var("APP_PORT").unwrap();
    let port = port.parse().expect("Port is Not a Number");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(authorization)
            .service(get_info)
            .service(refresh_token)
            .service(introspection)
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

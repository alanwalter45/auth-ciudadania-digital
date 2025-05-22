use actix_web::{HttpResponse, Responder, get, web};
use validator::Validate;

#[utoipa::path(
    get,
    tag = "API",
    path = "/",
    responses(
        (status = 200, description = "Redirect to authentication")
    ),
    params(
        ("url_client"=String, description="url's client to redirect"),
    )
)]
#[get("/")]
pub async fn origin(params: web::Query<ParamOrigin>) -> impl Responder {
    match params.validate() {
        Ok(_) => {
            // url_client no es retornado desde la API
            let url_client = "https://demo.chuquisaca.gob.bo";
            //let url_client = params.url_client.clone().unwrap();
            HttpResponse::Found()
                .insert_header(("Location", url_client))
                .finish()
        }
        Err(_) => HttpResponse::BadRequest().json("Arguments required not found."),
    }
}

#[utoipa::path(
    get,
    tag = "API",
    path = "/welcome",
    responses(
        (status = 200, description = "Redirect to client")
    ),
    params(
        ("code"=String, description="code get on authorization"),
        ("state"=String, description="state get on authorization"),
    )
)]
#[get("/welcome")]
pub async fn welcome(params: web::Query<ParamWelcome>) -> impl Responder {
    match params.validate() {
        Ok(_) => {
            let code = params.code.clone().unwrap();
            let state = params.state.clone().unwrap();
            // url_client no enviado en la peticion de retorno
            let url_client = "https://demo.chuquisaca.gob.bo/inicio";
            //let url_client = params.state.clone().unwrap();
            HttpResponse::Found()
                .insert_header((
                    "Location",
                    format!("{}?code={}&state={}", url_client, code, state),
                ))
                .finish()
        }
        Err(_) => HttpResponse::Found()
            .insert_header(("Location", "/"))
            .finish(),
    }
}

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
struct ParamWelcome {
    #[validate(
        required,
        length(min = 15, message = "code must be greater than 15 chars")
    )]
    code: Option<String>,
    #[validate(
        required,
        length(min = 15, message = "state must be greater than 15 chars")
    )]
    state: Option<String>,
    // #[validate(
    //     required,
    //     length(min = 8, message = "state must be greater than 8 chars")
    // )]
    // pub url_client: Option<String>,
}

#[derive(serde::Deserialize, utoipa::ToSchema, Validate)]
struct ParamOrigin {
    /* #[validate(
        required,
        length(min = 8, message = "state must be greater than 8 chars")
    )]
    pub url_client: Option<String>, */
}

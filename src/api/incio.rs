use crate::{
    AppState,
    api::{authorization::ParamAuthorization, introspection::*},
};
use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    tag = "API",
    path = "/inicio",
    responses(
        (status = 200, description = "Redirect to client")
    ),
    params(
        ("token"=ParamIntrospection, description="token get on authorization"),
    )
)]
#[get("/inicio")]
pub async fn inicio(
    data: web::Data<AppState>,
    params: web::Query<ParamAuthorization>,
) -> impl Responder {
    let code = params.code.clone().unwrap();
    let state = params.state.clone().unwrap();
    HttpResponse::Found()
        .insert_header((
            "Location",
            format!("{}/inicio?code={}&state={}", data.url_client, code, state),
        ))
        .finish()
}

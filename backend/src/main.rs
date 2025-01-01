use std::{env, net::Ipv4Addr};

use axum::{
    http::{HeaderValue, Method},
    Json,
};
use dotenv::dotenv;
use serde::Serialize;
use tokio::{io, net::TcpListener};
use tower_http::cors::{Any, CorsLayer};
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema, Serialize)]
struct Sample {
    data: String,
}
#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Hello World response", body = Sample)
    )
)]
async fn handler() -> Json<Sample> {
    Json(Sample {
        data: "Hello World".to_string(),
    })
}

#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv().ok();

    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let allow_origin = env::var("ORIGIN").expect("ORIGIN environment variable not set");

    let cors_layer = CorsLayer::new()
        .allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health))
        .routes(routes!(handler))
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 3000)).await?;
    axum::serve(listener, router).await
}

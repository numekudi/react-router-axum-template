use std::{env, net::Ipv4Addr};

use axum::http::{HeaderValue, Method};
use dotenv::dotenv;
use tokio::{io, net::TcpListener};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;
mod services;
use services::hello;
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
        .routes(routes!(hello::handler))
        .layer(cors_layer)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 3000)).await?;
    axum::serve(listener, router).await
}

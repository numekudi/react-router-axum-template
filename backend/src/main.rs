use std::env;

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
};
use axum::{
    http::{HeaderValue, Method},
    Extension, Json,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use schemars::JsonSchema;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize, Serialize, JsonSchema)]
struct Sample {
    data: String,
}

async fn root_handler() -> impl IntoApiResponse {
    Json(Sample {
        data: "Hello World".to_string(),
    })
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    dbg!(env::var("ORIGIN").unwrap());
    let allow_origin = env::var("ORIGIN").unwrap();
    let app = ApiRouter::new()
        .api_route("/hello", get(root_handler))
        .route("/api.json", get(serve_api))
        .layer(
            CorsLayer::new()
                .allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any),
        );

    let mut api = OpenApi {
        info: Info {
            description: Some("an example API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app.finish_api(&mut api)
            // Expose the documentation to the handlers.
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}

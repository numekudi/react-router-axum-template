use std::env;

use axum::{
    http::{HeaderValue, Method},
    response,
    routing::get,
    Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
#[derive(Deserialize, Serialize)]
struct Sample {
    data: String,
}

async fn root_handler() -> response::Json<Sample> {
    response::Json(Sample {
        data: "Hello World".to_string(),
    })
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let allow_origin = env::var("ORIGIN").unwrap();
    let app = Router::new().route("/", get(root_handler)).layer(
        CorsLayer::new()
            .allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(Any),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

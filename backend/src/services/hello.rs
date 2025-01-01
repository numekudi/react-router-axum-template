use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct Sample {
    data: String,
}

#[utoipa::path(
        get,
        path = "/hello",
        responses(
            (status = 200, description = "Hello World response", body = Sample)
    )
)]
pub async fn handler() -> Json<Sample> {
    Json(Sample {
        data: "Hello World".to_string(),
    })
}

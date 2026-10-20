use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

pub async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello from Tectonic2!".to_string(),
    })
}

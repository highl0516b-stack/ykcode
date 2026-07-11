use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn api_router() -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/version", get(version_info))
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "ykcode" }))
}

async fn version_info() -> Json<Value> {
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": "ykcode",
        "description": "Zero-Code Generation Platform"
    }))
}

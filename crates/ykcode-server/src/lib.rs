use std::sync::Arc;

use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use leptos_config::LeptosOptions;
use serde_json::{json, Value};
use ykcode_core::Document;
use ykcode_storage::{DocumentStore, StorageError};

// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<dyn DocumentStore + Send + Sync>,
    pub leptos_options: LeptosOptions,
}

// Required by leptos_routes and file_and_error_handler
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

// API error type
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl From<StorageError> for ApiError {
    fn from(e: StorageError) -> Self {
        match e {
            StorageError::NotFound(id) => ApiError::NotFound(id),
            other => ApiError::Internal(other.to_string()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            ApiError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/version", get(version_info))
        .route("/api/documents", get(list_documents))
        .route("/api/documents/{id}", get(get_document).put(save_document))
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "ykcode" }))
}

async fn version_info() -> Json<Value> {
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": "ykcode"
    }))
}

async fn list_documents(State(state): State<AppState>) -> Result<Json<Vec<String>>, ApiError> {
    let ids = state.store.list().map_err(ApiError::from)?;
    Ok(Json(ids))
}

async fn get_document(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Document>, ApiError> {
    let doc = state.store.load(&id).map_err(ApiError::from)?;
    Ok(Json(doc))
}

async fn save_document(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(doc): Json<Document>,
) -> Result<StatusCode, ApiError> {
    if doc.id.to_string() != id {
        return Err(ApiError::BadRequest("id mismatch".into()));
    }
    state.store.save(&doc).map_err(ApiError::from)?;
    Ok(StatusCode::NO_CONTENT)
}

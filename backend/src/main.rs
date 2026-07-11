use axum::{routing::get, Json, Router};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use serde_json::{json, Value};
use tower_http::trace::TraceLayer;
use ykcode_ui::{shell, App};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conf = get_configuration(None)?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let opts = leptos_options.clone();
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/version", get(version_info))
        .leptos_routes(&leptos_options, routes, move || shell(opts.clone()))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    eprintln!("ykcode listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

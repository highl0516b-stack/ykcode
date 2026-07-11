use anyhow::Context;
use app::{shell, App};
use axum::Router;
use leptos::config::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ── Tracing ───────────────────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("ykcode=debug".parse()?))
        .init();

    // ── Leptos configuration ──────────────────────────────────────────────────
    let conf =
        get_configuration(Some("Cargo.toml")).context("Failed to read Leptos configuration")?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    tracing::info!("ykcode server starting on {addr}");

    // ── Route list from Leptos app ────────────────────────────────────────────
    let routes = generate_route_list(App);

    // ── Axum router ──────────────────────────────────────────────────────────
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let options = leptos_options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    // ── Listen ────────────────────────────────────────────────────────────────
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind to {addr}"))?;

    tracing::info!("Listening on http://{addr}");

    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}

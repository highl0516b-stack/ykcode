use std::sync::Arc;

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::trace::TraceLayer;
use ykcode_server::{api_router, AppState};
use ykcode_storage::{native::FjallStore, DocumentStore};
use ykcode_ui::{shell, App};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    std::fs::create_dir_all("data")?;
    let store: Arc<dyn DocumentStore + Send + Sync> = Arc::new(FjallStore::open("data/ykcode")?);

    let conf = get_configuration(None)?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let state = AppState {
        store,
        leptos_options: leptos_options.clone(),
    };

    let opts = leptos_options.clone();
    let app = Router::new()
        .merge(api_router())
        .leptos_routes(&state, routes, move || shell(opts.clone()))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    eprintln!("ykcode listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

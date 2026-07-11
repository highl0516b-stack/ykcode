#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::compression::CompressionLayer;
    use tracing_subscriber::{fmt, EnvFilter};
    use zero_code_platform::app::{shell, App};

    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let opts = leptos_options.clone();
            move || shell(opts.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(CompressionLayer::new())
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Zero-Code Platform listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {}

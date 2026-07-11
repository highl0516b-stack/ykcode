pub mod components;

use components::editor::EditorShell;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="ykcode — Zero-Code Generation Platform"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="ykcode" href="/pkg/ykcode.css"/>
        <Title text="ykcode — Zero-Code Platform"/>
        <Meta name="theme-color" content="#090b10"/>

        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=EditorShell/>
            </Routes>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1 class="not-found__code">"404"</h1>
            <p class="not-found__msg">"Page not found."</p>
            <a href="/" class="not-found__link">"Return to editor"</a>
        </div>
    }
}

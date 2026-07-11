use crate::components::{Canvas, ComponentPalette, Inspector, LayersPanel, TopBar};
use crate::state::EditorState;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover" />
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
        <Stylesheet id="leptos" href="/pkg/zero-code-platform.css" />
        <Title text="Zero-Code Platform" />
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=EditorPage />
            </Routes>
        </Router>
    }
}

#[component]
fn EditorPage() -> impl IntoView {
    let state = EditorState::new();
    // Sync theme to <html> data-theme attribute via reactive Effect
    #[cfg(feature = "hydrate")]
    {
        let theme = state.theme;
        Effect::new(move |_| {
            use web_sys::window;
            if let Some(win) = window() {
                if let Some(doc) = win.document() {
                    if let Some(root) = doc.document_element() {
                        let _ = root.set_attribute("data-theme", theme.get());
                    }
                }
            }
        });
    }

    view! {
        <div class="editor-shell">
            <TopBar state=state />
            <ComponentPalette state=state />
            <Canvas state=state />
            <Inspector state=state />
            <LayersPanel state=state />
        </div>
    }
}

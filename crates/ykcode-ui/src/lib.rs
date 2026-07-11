mod autosave;
mod canvas;
mod dnd;
mod download;
mod editor;
mod history;
mod layers;
mod pages;
mod palette;
mod properties;
mod toolbar;

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use ykcode_core::{Display, Document, FlexDirection, Node, NodeId, Size};

use crate::editor::Editor;

pub(crate) use pages::PageStrip;

pub use history::{can_redo, can_undo, redo, undo, with_history};

// ---------------------------------------------------------------------------
// Editor context — shared reactive state
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub enum SaveStatus {
    Idle,
    Unsaved,
    Saving,
    Saved,
    Error(String),
}

impl SaveStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SaveStatus::Idle => "idle",
            SaveStatus::Unsaved => "unsaved",
            SaveStatus::Saving => "saving",
            SaveStatus::Saved => "saved",
            SaveStatus::Error(_) => "error",
        }
    }
}

#[derive(Clone, Copy)]
pub struct EditorCtx {
    pub left_panel_open: RwSignal<bool>,
    pub right_panel_open: RwSignal<bool>,
    pub active_left_tab: RwSignal<LeftTab>,
    pub zoom: RwSignal<f32>,
    pub selected_node: RwSignal<Option<NodeId>>,
    pub editing_node: RwSignal<Option<NodeId>>,
    pub document: RwSignal<Document>,
    pub drag_over_artboard: RwSignal<bool>,
    pub just_dropped: RwSignal<Option<NodeId>>,
    pub save_status: RwSignal<SaveStatus>,
    pub undo_stack: RwSignal<Vec<Document>>,
    pub redo_stack: RwSignal<Vec<Document>>,
    pub history_paused: RwSignal<bool>,
    pub publish_open: RwSignal<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LeftTab {
    Components,
    Layers,
}

pub(crate) fn node_inline_style(node: &Node) -> String {
    let mut s = Vec::new();

    // Display
    match node.layout.display {
        Display::Flex => s.push("display:flex".into()),
        Display::Grid => s.push("display:grid".into()),
        Display::Block => s.push("display:block".into()),
    }

    // Flex direction
    match node.layout.direction {
        FlexDirection::Row => s.push("flex-direction:row".into()),
        FlexDirection::Column => s.push("flex-direction:column".into()),
    }

    // Gap
    if node.layout.gap > 0.0 {
        s.push(format!("gap:{}px", node.layout.gap));
    }

    // Padding (only if non-zero)
    let p = &node.layout.padding;
    if p.top > 0.0 || p.right > 0.0 || p.bottom > 0.0 || p.left > 0.0 {
        s.push(format!(
            "padding:{}px {}px {}px {}px",
            p.top, p.right, p.bottom, p.left
        ));
    }

    // Width
    match &node.layout.width {
        Size::Fixed(v) => s.push(format!("width:{}px", v)),
        Size::Percent(v) => s.push(format!("width:{}%", v)),
        Size::Fill => s.push("width:100%".into()),
        Size::Auto => {}
    }

    // Height
    match &node.layout.height {
        Size::Fixed(v) => s.push(format!("height:{}px", v)),
        Size::Percent(v) => s.push(format!("height:{}%", v)),
        Size::Fill => s.push("height:100%".into()),
        Size::Auto => {}
    }

    // Background
    if let Some(bg) = &node.appearance.background {
        s.push(format!(
            "background:rgba({},{},{},{})",
            bg.r,
            bg.g,
            bg.b,
            bg.a as f32 / 255.0
        ));
    }

    // Opacity
    if node.appearance.opacity < 0.999 {
        s.push(format!("opacity:{:.3}", node.appearance.opacity));
    }

    s.join(";")
}

// ---------------------------------------------------------------------------
// Shell (SSR only)
// ---------------------------------------------------------------------------

#[cfg(feature = "ssr")]
pub fn shell(options: leptos_config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

// ---------------------------------------------------------------------------
// App — root with meta context and router
// ---------------------------------------------------------------------------

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ykcode.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700&family=Space+Grotesk:wght@400;500;600;700&display=swap"
        />
        <Title text="ykcode — Zero-Code Platform"/>
        <Router>
            <Routes fallback=|| view! { <p class="yk-not-found">"404 – Page not found"</p> }>
                <Route path=StaticSegment("") view=Editor/>
            </Routes>
        </Router>
    }
}

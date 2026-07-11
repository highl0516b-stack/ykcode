use leptos::prelude::*;

use crate::canvas::CanvasArea;
use crate::layers::LayerTree;
use crate::palette::ComponentPalette;
use crate::properties::PropertiesPanel;
use crate::toolbar::Toolbar;
use crate::{EditorCtx, LeftTab};

#[component]
pub(crate) fn Editor() -> impl IntoView {
    let ctx = EditorCtx {
        left_panel_open: RwSignal::new(true),
        right_panel_open: RwSignal::new(true),
        active_left_tab: RwSignal::new(LeftTab::Components),
        zoom: RwSignal::new(100.0f32),
        selected_node: RwSignal::new(None),
        editing_node: RwSignal::new(None),
        document: RwSignal::new(ykcode_core::Document::default()),
    };
    provide_context(ctx);

    view! {
        <div class="yk-shell">
            <Toolbar/>
            <div class="yk-workspace">
                <LeftPanel/>
                <CanvasArea/>
                <PropertiesPanel/>
            </div>
            <StatusBar/>
        </div>
    }
}

#[component]
fn LeftPanel() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");

    view! {
        <aside
            class="yk-left"
            class:yk-left--closed=move || !ctx.left_panel_open.get()
        >
            <div class="yk-panel-tabs">
                <button
                    class="yk-panel-tab"
                    class:yk-panel-tab--on=move || ctx.active_left_tab.get() == LeftTab::Components
                    on:click=move |_| ctx.active_left_tab.set(LeftTab::Components)
                >
                    "Components"
                </button>
                <button
                    class="yk-panel-tab"
                    class:yk-panel-tab--on=move || ctx.active_left_tab.get() == LeftTab::Layers
                    on:click=move |_| ctx.active_left_tab.set(LeftTab::Layers)
                >
                    "Layers"
                </button>
            </div>

            <div class="yk-panel-body">
                {move || match ctx.active_left_tab.get() {
                    LeftTab::Components => view! { <ComponentPalette/> }.into_any(),
                    LeftTab::Layers => view! { <LayerTree/> }.into_any(),
                }}
            </div>

            <button
                class="yk-rail-toggle"
                aria-label="Toggle panel"
                on:click=move |_| ctx.left_panel_open.update(|v| *v = !*v)
            >
                {move || if ctx.left_panel_open.get() { "‹" } else { "›" }}
            </button>
        </aside>
    }
}

#[component]
pub(crate) fn StatusBar() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");
    let node_count = move || {
        ctx.document.with(|d| {
            d.active_page()
                .and_then(|p| d.node(&p.root_node))
                .map(|n| n.children.len())
                .unwrap_or(0)
        })
    };

    view! {
        <footer class="yk-status">
            <span class="yk-status__l">"● Auto-saved"</span>
            <span class="yk-status__r">
                {move || {
                    let count = node_count();
                    if count == 0 {
                        "Empty page".into()
                    } else {
                        format!("{count} component{}", if count == 1 { "" } else { "s" })
                    }
                }}
            </span>
        </footer>
    }
}

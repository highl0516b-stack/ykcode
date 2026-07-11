use leptos::prelude::*;

use crate::canvas::CanvasArea;
use crate::layers::LayerTree;
use crate::palette::ComponentPalette;
use crate::properties::PropertiesPanel;
use crate::toolbar::Toolbar;
use crate::{EditorCtx, LeftTab, SaveStatus};

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
        drag_over_artboard: RwSignal::new(false),
        just_dropped: RwSignal::new(None),
        save_status: RwSignal::new(SaveStatus::Idle),
        undo_stack: RwSignal::new(Vec::new()),
        redo_stack: RwSignal::new(Vec::new()),
        history_paused: RwSignal::new(false),
    };
    provide_context(ctx);

    #[cfg(feature = "hydrate")]
    crate::autosave::provide_autosave(ctx);

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

    let save_label = move || match ctx.save_status.get() {
        SaveStatus::Idle => "All changes saved".to_string(),
        SaveStatus::Unsaved => "Unsaved changes".to_string(),
        SaveStatus::Saving => "Saving…".to_string(),
        SaveStatus::Saved => "Saved".to_string(),
        SaveStatus::Error(e) => format!("Error: {}", e),
    };

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
            <div
                class="yk-status__l yk-save"
                data-save-state=move || ctx.save_status.with(|s| s.as_str())
            >
                <span class="yk-save__mark" aria-hidden="true"/>
                <span class="yk-save__label" role="status" aria-live="polite">
                    {save_label}
                </span>
            </div>
            <span class="yk-status__r">
                {move || {
                    let n = node_count();
                    if n == 0 { "Empty page".into() }
                    else { format!("{n} component{}", if n == 1 { "" } else { "s" }) }
                }}
            </span>
        </footer>
    }
}

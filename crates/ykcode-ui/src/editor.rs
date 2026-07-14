use leptos::prelude::*;

use crate::canvas::CanvasArea;
use crate::layers::LayerTree;
use crate::palette::ComponentPalette;
use crate::properties::PropertiesPanel;
use crate::toolbar::Toolbar;
use crate::PageStrip;
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
        publish_open: RwSignal::new(false),
    };
    provide_context(ctx);

    #[cfg(feature = "hydrate")]
    crate::autosave::provide_autosave(ctx);

    view! {
        <div class="yk-shell">
            <Toolbar/>
            <PageStrip/>
            <div class="yk-workspace">
                <LeftPanel/>
                <CanvasArea/>
                <PropertiesPanel/>
            </div>
            <StatusBar/>
            {move || ctx.publish_open.get().then(|| view! { <PublishModal/> })}
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

#[component]
fn PublishModal() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");

    let page_count = move || ctx.document.with(|d| d.pages.len());
    let component_count = move || {
        ctx.document.with(|d| {
            d.active_page()
                .and_then(|p| d.node(&p.root_node))
                .map(|n| n.children.len())
                .unwrap_or(0)
        })
    };
    let doc_name = move || ctx.document.with(|d| d.name.clone());

    let close = move |_| ctx.publish_open.set(false);

    let download = move |_| {
        #[cfg(feature = "hydrate")]
        {
            let doc = ctx.document.get_untracked();
            if let Err(e) = crate::download::trigger_html_download(&doc) {
                leptos::logging::warn!("Export error: {:?}", e);
            }
        }
        ctx.publish_open.set(false);
    };

    view! {
        <div
            class="yk-publish-backdrop"
            on:click=close
        >
            <div
                class="yk-publish-modal"
                role="dialog"
                aria-modal="true"
                aria-labelledby="publish-title"
                on:click=|ev| ev.stop_propagation()
            >
                <header class="yk-publish-modal__header">
                    <h2 id="publish-title" class="yk-publish-modal__title">
                        "Export project"
                    </h2>
                    <button class="yk-btn yk-btn--ghost" on:click=close>"×"</button>
                </header>
                <div class="yk-publish-modal__body">
                    <dl class="yk-publish-modal__info">
                        <div>
                            <dt>"Project"</dt>
                            <dd>{doc_name}</dd>
                        </div>
                        <div>
                            <dt>"Pages"</dt>
                            <dd>{page_count}</dd>
                        </div>
                        <div>
                            <dt>"Components on active page"</dt>
                            <dd>{component_count}</dd>
                        </div>
                        <div>
                            <dt>"Format"</dt>
                            <dd>"Single HTML file (embedded CSS)"</dd>
                        </div>
                    </dl>
                </div>
                <footer class="yk-publish-modal__footer">
                    <button class="yk-btn yk-btn--secondary" on:click=close>
                        "Cancel"
                    </button>
                    <button class="yk-btn yk-btn--primary" on:click=download>
                        "⬇ Download HTML"
                    </button>
                </footer>
            </div>
        </div>
    }
}

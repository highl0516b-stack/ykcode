use leptos::prelude::*;
use shared::{Artboard, CanvasComponent, ComponentId, DisclosureLevel, Project, ViewportTransform};

use super::{
    canvas::CanvasViewport,
    palette::{ComponentPalette, PanelState},
    properties::PropertiesPanel,
    toolbar::MobileActionToolbar,
    topbar::TopCommandBar,
};

/// Root editor shell — implements the multi-panel layout from the UX spec.
#[component]
pub fn EditorShell() -> impl IntoView {
    // ── Project state ─────────────────────────────────────────────────────────
    let project = RwSignal::new(Project::new("My First Project"));

    // Derived signals via Signal::derive for ergonomic passing to children
    let project_name = Signal::derive(move || project.get().name.clone());
    let palette = Signal::derive(move || project.get().palette.clone());

    // ── Active artboard ───────────────────────────────────────────────────────
    let active_artboard_idx = RwSignal::new(0usize);
    let components = Signal::derive(move || {
        project
            .get()
            .artboards
            .get(active_artboard_idx.get())
            .cloned()
            .unwrap_or_else(|| Artboard::mobile("Mobile"))
            .components
    });

    // ── Selection ─────────────────────────────────────────────────────────────
    let (selected_id, set_selected_id) = signal::<Option<ComponentId>>(None);

    // ── Viewport transform ────────────────────────────────────────────────────
    let (transform, _set_transform) = signal(ViewportTransform::default());

    // ── Panel states ──────────────────────────────────────────────────────────
    let (palette_state, set_palette_state) = signal(PanelState::Docked);
    let (properties_open, set_properties_open) = signal(true);

    // ── Disclosure level ──────────────────────────────────────────────────────
    let (disclosure_level, set_disclosure_level) = signal(DisclosureLevel::Guided);

    // ── History (simple undo stack) ───────────────────────────────────────────
    let history = RwSignal::<Vec<Vec<CanvasComponent>>>::new(Vec::new());

    let push_history = move || {
        let snap = components.get();
        history.update(|h| {
            h.push(snap);
            if h.len() > 50 {
                h.remove(0);
            }
        });
    };

    let undo = move || {
        history.update(|h| {
            if let Some(prev) = h.pop() {
                project.update(|p| {
                    if let Some(ab) = p.artboards.get_mut(active_artboard_idx.get()) {
                        ab.components = prev;
                    }
                });
            }
        });
    };

    // ── Insert component ──────────────────────────────────────────────────────
    let insert_component = move |c: CanvasComponent| {
        push_history();
        project.update(|p| {
            if let Some(ab) = p.artboards.get_mut(active_artboard_idx.get()) {
                ab.components.push(c);
            }
        });
    };

    view! {
        <div class="editor-shell">
            // ── Top command bar ───────────────────────────────────────────────
            <TopCommandBar
                project_name=project_name
                disclosure_level=disclosure_level
                set_disclosure_level=set_disclosure_level
            />

            // ── Main workspace ────────────────────────────────────────────────
            <div class="editor-workspace">
                // Left: Component palette
                <ComponentPalette
                    state=palette_state
                    on_insert=Callback::new(insert_component)
                />

                // Center: Canvas
                <main class="editor-main">
                    <CanvasViewport
                        components=components
                        selected_id=selected_id
                        set_selected_id=set_selected_id
                        transform=transform
                    />
                </main>

                // Right: Properties panel
                <Show when=move || properties_open.get()>
                    <PropertiesPanel
                        components=components
                        selected_id=selected_id
                        palette=palette
                    />
                </Show>
            </div>

            // ── Mobile floating toolbar ───────────────────────────────────────
            <MobileActionToolbar
                on_undo=Callback::new(move |_| undo())
                on_redo=Callback::new(|_| {})
                on_open_palette=Callback::new(move |_| {
                    set_palette_state.update(|s| {
                        *s = if *s == PanelState::Overlay {
                            PanelState::Hidden
                        } else {
                            PanelState::Overlay
                        };
                    });
                })
                on_open_layers=Callback::new(|_| {})
                on_open_properties=Callback::new(move |_| {
                    set_properties_open.update(|v| *v = !*v);
                })
            />
        </div>
    }
}

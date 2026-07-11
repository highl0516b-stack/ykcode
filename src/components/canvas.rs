use crate::state::{ActiveTool, CanvasElement, CanvasTransform, ComponentKind, EditorState};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn Canvas(state: EditorState) -> impl IntoView {
    let transform = state.canvas_transform;
    let elements = state.elements;
    let selected_ids = state.selected_ids;
    let tool = state.active_tool;
    let dragging_kind = state.dragging_kind;

    let state_drop = state;
    let state_click = state;

    let transform_style = move || {
        let t = transform.get();
        format!("transform: {}", t.css_transform())
    };

    let canvas_class = move || match tool.get() {
        ActiveTool::Hand => "canvas-container hand-tool",
        ActiveTool::Select => "canvas-container",
    };

    view! {
        <main
            class=canvas_class
            on:dragover=move |ev| {
                ev.prevent_default();
                if let Some(dt) = ev.data_transfer() {
                    dt.set_drop_effect("copy");
                }
            }
            on:drop=move |ev| {
                ev.prevent_default();
                let kind = state_drop.dragging_kind.get_untracked();
                if let Some(kind) = kind {
                    let t = transform.get_untracked();
                    let (cx, cy) = screen_to_canvas(
                        ev.offset_x() as f64,
                        ev.offset_y() as f64,
                        &t,
                    );
                    state_drop.drop_component(kind, cx, cy);
                }
            }
            on:click=move |ev| {
                if ev.target() == ev.current_target() {
                    state_click.clear_selection();
                }
            }
            on:wheel=move |ev| {
                ev.prevent_default();
                let t = transform.get_untracked();
                let delta = -ev.delta_y() / 500.0;
                let new_t =
                    t.apply_zoom(delta, ev.offset_x() as f64, ev.offset_y() as f64);
                state.canvas_transform.set(new_t);
            }
        >
            // Canvas viewport (transformed layer)
            <div class="canvas-viewport" style=transform_style>
                // Primary artboard
                <div
                    class="canvas-artboard"
                    style="width:1440px;height:900px;left:50%;top:50%;transform:translate(-50%,-50%);"
                >
                    // Render placed elements reactively
                    {move || {
                        elements
                            .get()
                            .into_iter()
                            .map(|el| {
                                view! { <PlacedElement el=el selected_ids=selected_ids state=state /> }
                            })
                            .collect_view()
                    }}
                </div>
            </div>

            // Empty state hint
            <Show when=move || elements.get().is_empty() && dragging_kind.get().is_none()>
                <div class="canvas-empty">
                    <div class="empty-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                            <rect x="3" y="3" width="18" height="18" rx="3" />
                            <path d="M12 8v8M8 12h8" />
                        </svg>
                    </div>
                    <p class="empty-title">"Start creating"</p>
                    <p class="empty-subtitle">
                        "Drag a component from the left panel onto the canvas"
                    </p>
                </div>
            </Show>

            // Floating zoom pill
            <ZoomPill transform=transform />
        </main>
    }
}

#[component]
fn PlacedElement(
    el: CanvasElement,
    selected_ids: RwSignal<Vec<Uuid>>,
    state: EditorState,
) -> impl IntoView {
    let id = el.id;
    let is_selected = move || selected_ids.get().contains(&id);
    let kind_label = el.kind.label();
    let preview_style = element_preview_style(&el.kind);

    let style = format!(
        "position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;z-index:{};",
        el.x, el.y, el.width, el.height, el.z_index
    );

    view! {
        <div
            class=move || {
                if is_selected() { "placed-element selected" } else { "placed-element" }
            }
            style=style
            on:click=move |ev| {
                ev.stop_propagation();
                state.select(id, ev.shift_key());
            }
        >
            <div
                class="element-preview"
                style=format!(
                    "width:100%;height:100%;display:flex;align-items:center;justify-content:center;{}",
                    preview_style,
                )
            >
                <span style="font-size:12px;color:var(--color-text-tertiary);pointer-events:none;">
                    {kind_label}
                </span>
            </div>

            // Selection handles (only when selected)
            <Show when=is_selected>
                <div
                    style="position:absolute;inset:0;pointer-events:none;"
                    class="selection-overlay"
                >
                    <div class="selection-bounds" style="inset:0;" />
                    <div class="selection-handle corner-tl" />
                    <div class="selection-handle corner-tr" />
                    <div class="selection-handle corner-bl" />
                    <div class="selection-handle corner-br" />
                    <div class="selection-handle rotate" />
                </div>
            </Show>
        </div>
    }
}

#[component]
fn ZoomPill(transform: RwSignal<CanvasTransform>) -> impl IntoView {
    let zoom_pct = move || (transform.get().zoom * 100.0).round() as u32;

    view! {
        <div class="canvas-zoom-pill">
            <button
                class="btn-icon"
                style="min-width:1.5rem;min-height:1.5rem;"
                title="Zoom out"
                on:click=move |_| {
                    transform.update(|t| {
                        t.zoom = (t.zoom / 1.25).max(CanvasTransform::ZOOM_MIN);
                    });
                }
            >
                <svg
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <circle cx="11" cy="11" r="8" />
                    <line x1="8" y1="11" x2="14" y2="11" />
                </svg>
            </button>

            <span
                class="zoom-value"
                title="Click to reset to 100%"
                on:click=move |_| {
                    transform.update(|t| {
                        t.zoom = 1.0;
                        t.offset_x = 0.0;
                        t.offset_y = 0.0;
                    });
                }
            >
                {move || format!("{}%", zoom_pct())}
            </span>

            <button
                class="btn-icon"
                style="min-width:1.5rem;min-height:1.5rem;"
                title="Zoom in"
                on:click=move |_| {
                    transform.update(|t| {
                        t.zoom = (t.zoom * 1.25).min(CanvasTransform::ZOOM_MAX);
                    });
                }
            >
                <svg
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <circle cx="11" cy="11" r="8" />
                    <line x1="11" y1="8" x2="11" y2="14" />
                    <line x1="8" y1="11" x2="14" y2="11" />
                </svg>
            </button>
        </div>
    }
}

fn screen_to_canvas(screen_x: f64, screen_y: f64, t: &CanvasTransform) -> (f64, f64) {
    let canvas_x = (screen_x - t.offset_x) / t.zoom;
    let canvas_y = (screen_y - t.offset_y) / t.zoom;
    (canvas_x, canvas_y)
}

fn element_preview_style(kind: &ComponentKind) -> &'static str {
    match kind {
        ComponentKind::Button => {
            "background:var(--color-brand-primary);border-radius:var(--radius-md);"
        }
        ComponentKind::TextBlock => "border-bottom:2px solid var(--color-border-default);",
        ComponentKind::Image => {
            "background:var(--color-surface-sunken);border:1px dashed var(--color-border-default);"
        }
        ComponentKind::Container => {
            "border:1px dashed var(--color-brand-primary);border-radius:var(--radius-md);"
        }
        ComponentKind::Input => {
            "background:var(--color-surface-sunken);border:1px solid var(--color-border-default);border-radius:var(--radius-sm);"
        }
        ComponentKind::Divider => "background:var(--color-border-default);height:1px;",
        ComponentKind::Spacer => {
            "background:repeating-linear-gradient(45deg,var(--color-border-subtle) 0,var(--color-border-subtle) 1px,transparent 0,transparent 50%) 0 0/8px 8px;"
        }
        ComponentKind::NavigationBar => {
            "background:var(--color-surface);border-bottom:1px solid var(--color-border-subtle);"
        }
        ComponentKind::Card => {
            "background:var(--color-surface-raised);border-radius:var(--radius-lg);box-shadow:var(--shadow-sm);"
        }
        ComponentKind::List => {
            "background:var(--color-surface);border:1px solid var(--color-border-subtle);border-radius:var(--radius-md);"
        }
        ComponentKind::Form => {
            "background:var(--color-surface-sunken);border:1px solid var(--color-border-default);border-radius:var(--radius-md);"
        }
        ComponentKind::Modal => {
            "background:var(--color-surface-raised);border-radius:var(--radius-xl);box-shadow:var(--shadow-lg);"
        }
    }
}

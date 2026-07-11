use crate::state::{EditorState, PanelState};
use leptos::children::ChildrenFn;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn Inspector(state: EditorState) -> impl IntoView {
    let panel_state = state.panel_right;
    let selected_ids = state.selected_ids;
    let elements = state.elements;

    let panel_class = move || match panel_state.get() {
        PanelState::Expanded => "panel-right",
        PanelState::Collapsed => "panel-right collapsed",
        PanelState::Hidden => "panel-right hidden",
    };

    let selected_element = move || {
        let ids = selected_ids.get();
        let first = *ids.first()?;
        elements.get().into_iter().find(|e| e.id == first)
    };

    let state_inner = state;

    view! {
        <aside class=panel_class>
            <InspectorHeader panel_state=panel_state />
            <Show when=move || panel_state.get() == PanelState::Expanded>
                <div class="panel__content">
                    {move || {
                        if let Some(el) = selected_element() {
                            view! {
                                <ElementInspector el_id=el.id state=state_inner />
                            }
                            .into_any()
                        } else {
                            view! { <NoSelectionHint /> }.into_any()
                        }
                    }}
                </div>
            </Show>
        </aside>
    }
}

#[component]
fn InspectorHeader(panel_state: RwSignal<PanelState>) -> impl IntoView {
    view! {
        <div class="panel__header">
            <button
                class="btn-icon"
                title="Toggle inspector"
                on:click=move |_| {
                    panel_state.update(|s| {
                        *s = match *s {
                            PanelState::Expanded => PanelState::Collapsed,
                            PanelState::Collapsed => PanelState::Expanded,
                            PanelState::Hidden => PanelState::Expanded,
                        };
                    });
                }
            >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    {move || {
                        if panel_state.get() == PanelState::Expanded {
                            view! { <path d="M9 18l6-6-6-6" /> }.into_any()
                        } else {
                            view! { <path d="M15 18l-6-6 6-6" /> }.into_any()
                        }
                    }}
                </svg>
            </button>
            <span class="panel__title">"Inspect"</span>
        </div>
    }
}

#[component]
fn ElementInspector(el_id: Uuid, state: EditorState) -> impl IntoView {
    let elements = state.elements;
    let state_delete = state;

    let el_data = move || elements.get().into_iter().find(|e| e.id == el_id);
    let short_id = el_id.to_string();
    let short_id = short_id.split('-').next().unwrap_or("").to_string();

    view! {
        {move || {
            el_data().map(|el| {
                let kind_label = el.kind.label();
                let el_x = el.x.round() as i64;
                let el_y = el.y.round() as i64;
                let el_w = el.width.round() as i64;
                let el_h = el.height.round() as i64;
                let short = short_id.clone();
                let state_del = state_delete;

                view! {
                    // Header
                    <div style="padding:var(--space-3) var(--space-4);border-bottom:1px solid var(--color-border-subtle);">
                        <div style="display:flex;align-items:center;justify-content:space-between;">
                            <span style="font-size:var(--font-size-sm);font-weight:var(--font-weight-semibold);color:var(--color-text-primary);">
                                {kind_label}
                            </span>
                            <button
                                class="btn-icon"
                                title="Delete element"
                                style="color:var(--color-error);"
                                on:click=move |_| state_del.delete_selected()
                            >
                                <svg
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <polyline points="3 6 5 6 21 6" />
                                    <path d="M19 6l-1 14H6L5 6" />
                                    <path d="M10 11v6M14 11v6" />
                                    <path d="M9 6V4h6v2" />
                                </svg>
                            </button>
                        </div>
                        <span style="font-size:var(--font-size-xs);color:var(--color-text-tertiary);font-family:var(--font-mono);">
                            {short}
                        </span>
                    </div>

                    // Layout section
                    <InspectorSection title="Layout">
                        <div class="property-row">
                            <span class="property-label">"X"</span>
                            <input class="property-input" type="number" value=el_x readonly />
                        </div>
                        <div class="property-row">
                            <span class="property-label">"Y"</span>
                            <input class="property-input" type="number" value=el_y readonly />
                        </div>
                        <div class="property-row">
                            <span class="property-label">"W"</span>
                            <input class="property-input" type="number" value=el_w readonly />
                        </div>
                        <div class="property-row">
                            <span class="property-label">"H"</span>
                            <input class="property-input" type="number" value=el_h readonly />
                        </div>
                    </InspectorSection>

                    // Fill section
                    <InspectorSection title="Fill">
                        <div class="color-swatch">
                            <div
                                class="swatch-preview"
                                style="background:var(--color-brand-primary);"
                            />
                            <span class="swatch-value">"#6657F5"</span>
                        </div>
                    </InspectorSection>

                    // Typography section
                    <InspectorSection title="Typography">
                        <div class="property-row">
                            <span class="property-label">"Font"</span>
                            <input class="property-input" type="text" value="Inter" readonly />
                        </div>
                        <div class="property-row">
                            <span class="property-label">"Size"</span>
                            <input class="property-input" type="number" value=14 readonly />
                        </div>
                    </InspectorSection>
                }
            })
        }}
    }
}

#[component]
fn InspectorSection(title: &'static str, children: ChildrenFn) -> impl IntoView {
    let expanded = RwSignal::new(true);

    view! {
        <div class="inspector-section">
            <div class="section__header" on:click=move |_| expanded.update(|e| *e = !*e)>
                <span class="section__title">{title}</span>
                <svg
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    style=move || {
                        if expanded.get() {
                            "transition:transform 0.2s"
                        } else {
                            "transform:rotate(-90deg);transition:transform 0.2s"
                        }
                    }
                >
                    <path d="M19 9l-7 7-7-7" />
                </svg>
            </div>
            <Show when=move || expanded.get()>
                <div class="section__body">{children()}</div>
            </Show>
        </div>
    }
}

#[component]
fn NoSelectionHint() -> impl IntoView {
    view! {
        <div style="padding:var(--space-8) var(--space-4);display:flex;flex-direction:column;align-items:center;gap:var(--space-3);text-align:center;">
            <svg
                width="32"
                height="32"
                viewBox="0 0 24 24"
                fill="none"
                stroke="var(--color-border-strong)"
                stroke-width="1.5"
            >
                <rect x="3" y="3" width="18" height="18" rx="3" />
                <path d="M3 9h18M9 21V9" />
            </svg>
            <p style="font-size:var(--font-size-sm);color:var(--color-text-tertiary);">
                "Select an element to inspect its properties"
            </p>
        </div>
    }
}

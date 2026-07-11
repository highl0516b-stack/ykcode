use crate::state::{CanvasElement, EditorState, PanelState};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn LayersPanel(state: EditorState) -> impl IntoView {
    let panel_state = state.panel_bottom;
    let elements = state.elements;
    let selected_ids = state.selected_ids;
    let active_tab = RwSignal::new(0usize);
    let state_list = state;

    view! {
        <div class="panel-bottom">
            // Drag handle / collapse toggle
            <div
                class="panel__handle"
                on:click=move |_| {
                    panel_state.update(|s| {
                        *s = match *s {
                            PanelState::Expanded => PanelState::Collapsed,
                            _ => PanelState::Expanded,
                        };
                    });
                }
            >
                <span style="font-size:var(--font-size-xs);color:var(--color-text-tertiary);">
                    {move || {
                        let count = elements.get().len();
                        format!("{} layer{}", count, if count == 1 { "" } else { "s" })
                    }}
                </span>
                <svg
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    style=move || {
                        if panel_state.get() == PanelState::Expanded {
                            "transform:rotate(180deg);transition:transform 0.2s"
                        } else {
                            "transition:transform 0.2s"
                        }
                    }
                >
                    <path d="M18 15l-6-6-6 6" />
                </svg>
            </div>

            <Show when=move || panel_state.get() == PanelState::Expanded>
                // Tab bar
                <div class="panel__tabs">
                    <button
                        class=move || if active_tab.get() == 0 { "tab active" } else { "tab" }
                        on:click=move |_| active_tab.set(0)
                    >
                        "Layers"
                    </button>
                    <button
                        class=move || if active_tab.get() == 1 { "tab active" } else { "tab" }
                        on:click=move |_| active_tab.set(1)
                    >
                        "Interactions"
                    </button>
                    <button
                        class=move || if active_tab.get() == 2 { "tab active" } else { "tab" }
                        on:click=move |_| active_tab.set(2)
                    >
                        "Timeline"
                    </button>
                </div>

                <div class="panel__content">
                    <Show
                        when=move || active_tab.get() == 0
                        fallback=|| {
                            view! {
                                <div style="padding:var(--space-4);color:var(--color-text-tertiary);font-size:var(--font-size-xs);text-align:center;">
                                    "Coming soon"
                                </div>
                            }
                        }
                    >
                        <LayerList
                            elements=elements
                            selected_ids=selected_ids
                            state=state_list
                        />
                    </Show>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn LayerList(
    elements: RwSignal<Vec<CanvasElement>>,
    selected_ids: RwSignal<Vec<Uuid>>,
    state: EditorState,
) -> impl IntoView {
    view! {
        <div>
            {move || {
                let mut els = elements.get();
                if els.is_empty() {
                    return view! {
                        <div style="padding:var(--space-4);color:var(--color-text-tertiary);font-size:var(--font-size-xs);text-align:center;">
                            "No layers yet — drag components onto the canvas"
                        </div>
                    }
                    .into_any();
                }
                els.reverse();
                let rows = els
                    .into_iter()
                    .map(|el| {
                        let id = el.id;
                        let label = el.label.clone();
                        let icon_path = el.kind.icon_path();
                        let state_row = state;
                        view! {
                            <LayerRow
                                id=id
                                label=label
                                icon_path=icon_path
                                selected_ids=selected_ids
                                state=state_row
                            />
                        }
                    })
                    .collect_view();
                view! { <div>{rows}</div> }.into_any()
            }}
        </div>
    }
}

#[component]
fn LayerRow(
    id: Uuid,
    label: String,
    icon_path: &'static str,
    selected_ids: RwSignal<Vec<Uuid>>,
    state: EditorState,
) -> impl IntoView {
    let is_selected = move || selected_ids.get().contains(&id);

    view! {
        <div
            class=move || if is_selected() { "layer-row selected" } else { "layer-row" }
            on:click=move |ev| state.select(id, ev.shift_key())
        >
            <svg
                class="layer-icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.5"
            >
                <path d=icon_path />
            </svg>
            <span class="layer-name">{label}</span>
        </div>
    }
}

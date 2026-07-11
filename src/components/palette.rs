use crate::state::{ComponentKind, EditorState, PanelState};
use leptos::prelude::*;

// ── Static list of (category_name, component kinds) ──────────────────────────
fn all_categories() -> Vec<(&'static str, Vec<ComponentKind>)> {
    vec![
        (
            "Controls",
            vec![ComponentKind::Button, ComponentKind::Input],
        ),
        ("Typography", vec![ComponentKind::TextBlock]),
        (
            "Layout",
            vec![
                ComponentKind::Container,
                ComponentKind::Divider,
                ComponentKind::Spacer,
            ],
        ),
        ("Media", vec![ComponentKind::Image, ComponentKind::Card]),
        ("Navigation", vec![ComponentKind::NavigationBar]),
        (
            "Forms",
            vec![
                ComponentKind::List,
                ComponentKind::Form,
                ComponentKind::Modal,
            ],
        ),
    ]
}

#[component]
pub fn ComponentPalette(state: EditorState) -> impl IntoView {
    let panel_state = state.panel_left;

    let panel_class = move || match panel_state.get() {
        PanelState::Expanded => "panel-left",
        PanelState::Collapsed => "panel-left collapsed",
        PanelState::Hidden => "panel-left hidden",
    };

    view! {
        <aside class=panel_class>
            <PaletteHeader panel_state=panel_state />
            <Show when=move || panel_state.get() == PanelState::Expanded>
                <div class="panel__search">
                    <input
                        type="search"
                        placeholder="Search components…"
                        aria-label="Search components"
                    />
                </div>
                <div class="panel__content">
                    {all_categories()
                        .into_iter()
                        .map(|(name, kinds)| {
                            view! { <CategoryGroup name=name kinds=kinds state=state /> }
                        })
                        .collect_view()}
                </div>
            </Show>
        </aside>
    }
}

#[component]
fn PaletteHeader(panel_state: RwSignal<PanelState>) -> impl IntoView {
    view! {
        <div class="panel__header">
            <span class="panel__title">"Components"</span>
            <button
                class="btn-icon"
                title="Toggle palette"
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
                            view! { <path d="M15 18l-6-6 6-6" /> }.into_any()
                        } else {
                            view! { <path d="M9 18l6-6-6-6" /> }.into_any()
                        }
                    }}
                </svg>
            </button>
        </div>
    }
}

#[component]
fn CategoryGroup(
    name: &'static str,
    kinds: Vec<ComponentKind>,
    state: EditorState,
) -> impl IntoView {
    let collapsed = RwSignal::new(false);
    // Store kinds in a signal so they can be accessed in the reactive closure
    let kinds = StoredValue::new(kinds);

    view! {
        <div class="component-category">
            <div class="category__header" on:click=move |_| collapsed.update(|c| *c = !*c)>
                <span class="category__name">{name}</span>
                <svg
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    style=move || {
                        if collapsed.get() {
                            "transform:rotate(-90deg);transition:transform 0.2s"
                        } else {
                            "transition:transform 0.2s"
                        }
                    }
                >
                    <path d="M19 9l-7 7-7-7" />
                </svg>
            </div>
            {move || {
                if collapsed.get() {
                    None
                } else {
                    let cards = kinds
                        .get_value()
                        .into_iter()
                        .map(|kind| view! { <ComponentCard kind=kind state=state /> })
                        .collect_view();
                    Some(view! { <div class="category__grid">{cards}</div> })
                }
            }}
        </div>
    }
}

#[component]
fn ComponentCard(kind: ComponentKind, state: EditorState) -> impl IntoView {
    let label = kind.label();
    let icon_path = kind.icon_path();
    let is_dragging = RwSignal::new(false);
    let dragging_kind = state.dragging_kind;
    let is_dragging_global = state.is_dragging;
    let kind_for_drag = kind.clone();

    view! {
        <div
            class=move || if is_dragging.get() { "component-card dragging" } else { "component-card" }
            draggable="true"
            title=format!("Drag to add {}", label)
            on:dragstart=move |ev| {
                is_dragging.set(true);
                is_dragging_global.set(true);
                dragging_kind.set(Some(kind_for_drag.clone()));
                if let Some(dt) = ev.data_transfer() {
                    let _ = dt.set_data("text/plain", label);
                    dt.set_drop_effect("copy");
                }
            }
            on:dragend=move |_| {
                is_dragging.set(false);
                is_dragging_global.set(false);
                dragging_kind.set(None);
            }
        >
            <div class="card__preview">
                <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d=icon_path />
                </svg>
            </div>
            <span class="card__name">{label}</span>
        </div>
    }
}

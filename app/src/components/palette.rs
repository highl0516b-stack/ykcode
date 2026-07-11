use leptos::prelude::*;
use shared::{Bounds, CanvasComponent, ComponentKind};

/// Panel state following the 5-state model from the UX spec.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PanelState {
    #[default]
    Hidden,
    Collapsed,
    Overlay,
    Docked,
    Pinned,
}

#[derive(Debug, Clone)]
struct ComponentEntry {
    kind: ComponentKind,
    label: &'static str,
    category: &'static str,
    description: &'static str,
}

fn component_catalogue() -> Vec<ComponentEntry> {
    vec![
        ComponentEntry {
            kind: ComponentKind::Button,
            label: "Button",
            category: "Basics",
            description: "Clickable action trigger",
        },
        ComponentEntry {
            kind: ComponentKind::Text,
            label: "Text",
            category: "Basics",
            description: "Paragraph or heading",
        },
        ComponentEntry {
            kind: ComponentKind::Image,
            label: "Image",
            category: "Basics",
            description: "Raster or SVG image",
        },
        ComponentEntry {
            kind: ComponentKind::Icon,
            label: "Icon",
            category: "Basics",
            description: "Scalable icon",
        },
        ComponentEntry {
            kind: ComponentKind::Container,
            label: "Container",
            category: "Layout",
            description: "Block-level wrapper",
        },
        ComponentEntry {
            kind: ComponentKind::Row,
            label: "Row",
            category: "Layout",
            description: "Horizontal flex row",
        },
        ComponentEntry {
            kind: ComponentKind::Column,
            label: "Column",
            category: "Layout",
            description: "Vertical flex column",
        },
        ComponentEntry {
            kind: ComponentKind::Grid,
            label: "Grid",
            category: "Layout",
            description: "CSS grid layout",
        },
        ComponentEntry {
            kind: ComponentKind::Input,
            label: "Input",
            category: "Forms",
            description: "Text input field",
        },
        ComponentEntry {
            kind: ComponentKind::Textarea,
            label: "Textarea",
            category: "Forms",
            description: "Multi-line text area",
        },
        ComponentEntry {
            kind: ComponentKind::Select,
            label: "Select",
            category: "Forms",
            description: "Dropdown select",
        },
        ComponentEntry {
            kind: ComponentKind::Toggle,
            label: "Toggle",
            category: "Forms",
            description: "Boolean toggle switch",
        },
        ComponentEntry {
            kind: ComponentKind::Card,
            label: "Card",
            category: "Data",
            description: "Elevated content card",
        },
        ComponentEntry {
            kind: ComponentKind::Table,
            label: "Table",
            category: "Data",
            description: "Structured data table",
        },
        ComponentEntry {
            kind: ComponentKind::Navbar,
            label: "Navbar",
            category: "Navigation",
            description: "Top navigation bar",
        },
    ]
}

static CATEGORIES: &[&str] = &["All", "Basics", "Layout", "Forms", "Data", "Navigation"];

#[component]
pub fn ComponentPalette(
    state: ReadSignal<PanelState>,
    on_insert: Callback<CanvasComponent>,
) -> impl IntoView {
    let search = RwSignal::new(String::new());
    let active_category = RwSignal::new("All");

    let catalogue = component_catalogue();

    let filtered = move || {
        let q = search.get().to_lowercase();
        let cat = active_category.get();
        catalogue
            .iter()
            .filter(|e| {
                let matches_cat = cat == "All" || e.category == cat;
                let matches_q = q.is_empty()
                    || e.label.to_lowercase().contains(&q)
                    || e.description.to_lowercase().contains(&q);
                matches_cat && matches_q
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    view! {
        <aside
            class="palette-panel glass-panel"
            class:hidden=move || state.get() == PanelState::Hidden
            class:collapsed=move || state.get() == PanelState::Collapsed
            aria-label="Component palette"
        >
            // Search
            <div class="palette-panel__search">
                <svg class="palette-panel__search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75">
                    <circle cx="11" cy="11" r="8"/>
                    <path d="M21 21l-4.35-4.35"/>
                </svg>
                <input
                    class="palette-panel__search-input"
                    type="search"
                    placeholder="Search components…"
                    prop:value=search
                    on:input=move |e| {
                        search.set(event_target_value(&e));
                    }
                    aria-label="Search components"
                />
            </div>

            // Category tabs
            <div class="palette-panel__tabs" role="tablist">
                {CATEGORIES.iter().map(|&cat| {
                    let is_active = move || active_category.get() == cat;
                    view! {
                        <button
                            class="palette-panel__tab"
                            class:active=is_active
                            role="tab"
                            aria-selected=move || is_active().to_string()
                            on:click=move |_| active_category.set(cat)
                        >
                            {cat}
                        </button>
                    }
                }).collect_view()}
            </div>

            // Component grid
            <div class="palette-panel__grid" role="listbox">
                {move || filtered().into_iter().map(|entry| {
                    let kind_clone = entry.kind.clone();
                    view! {
                        <ComponentChip
                            label=entry.label
                            description=entry.description
                            on_insert=Callback::new(move |_| {
                                let c = CanvasComponent::new(
                                    kind_clone.clone(),
                                    entry.label,
                                    Bounds::new(40.0, 40.0, 160.0, 48.0),
                                );
                                on_insert.run(c);
                            })
                        />
                    }
                }).collect_view()}
            </div>
        </aside>
    }
}

#[component]
fn ComponentChip(
    label: &'static str,
    description: &'static str,
    on_insert: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            class="component-chip"
            role="option"
            title=description
            on:click=move |_| on_insert.run(())
            aria-label=format!("Insert {}", label)
        >
            <div class="component-chip__thumb" aria-hidden="true">
                <span class="component-chip__icon">{&label[..1]}</span>
            </div>
            <span class="component-chip__label">{label}</span>
        </button>
    }
}

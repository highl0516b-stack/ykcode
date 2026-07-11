use leptos::prelude::*;
use ykcode_core::{Node, NodeKind};

use crate::EditorCtx;

#[derive(Clone, PartialEq)]
struct PaletteEntry {
    icon: &'static str,
    label: &'static str,
    hint: &'static str,
    featured: bool,
}

fn all_palette_entries() -> Vec<PaletteEntry> {
    vec![
        PaletteEntry {
            icon: "⬜",
            label: "Section",
            hint: "Full-width layout block",
            featured: false,
        },
        PaletteEntry {
            icon: "≡",
            label: "Stack",
            hint: "Flex row or column",
            featured: false,
        },
        PaletteEntry {
            icon: "T",
            label: "Text",
            hint: "Heading or paragraph",
            featured: false,
        },
        PaletteEntry {
            icon: "◉",
            label: "Button",
            hint: "Call to action",
            featured: true,
        },
        PaletteEntry {
            icon: "🖼",
            label: "Image",
            hint: "Photo or illustration",
            featured: false,
        },
        PaletteEntry {
            icon: "▭",
            label: "Container",
            hint: "Nestable block",
            featured: false,
        },
        PaletteEntry {
            icon: "—",
            label: "Divider",
            hint: "Horizontal rule",
            featured: false,
        },
        PaletteEntry {
            icon: "↕",
            label: "Spacer",
            hint: "Flexible gap",
            featured: false,
        },
    ]
}

fn kind_from_label(label: &str) -> NodeKind {
    match label {
        "Section" => NodeKind::Section,
        "Stack" => NodeKind::Stack,
        "Text" => NodeKind::Text,
        "Button" => NodeKind::Button,
        "Image" => NodeKind::Image,
        "Container" => NodeKind::Container,
        "Divider" => NodeKind::Divider,
        "Spacer" => NodeKind::Spacer,
        _ => NodeKind::Container,
    }
}

fn node_with_defaults(kind: NodeKind) -> Node {
    let mut node = Node::new(kind.clone());
    node.content = match kind {
        NodeKind::Text => Some("Add your text".into()),
        NodeKind::Button => Some("Button".into()),
        NodeKind::Image => Some("🖼 Add image".into()),
        _ => None,
    };
    node
}

#[component]
pub(crate) fn ComponentPalette() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");
    let query = RwSignal::new(String::new());
    let entries = all_palette_entries();

    let filtered = move || {
        let q = query.get().to_lowercase();
        entries
            .iter()
            .filter(|e| q.is_empty() || e.label.to_lowercase().contains(&q))
            .cloned()
            .collect::<Vec<_>>()
    };

    view! {
        <div class="yk-palette">
            <div class="yk-palette__search">
                <input
                    type="search"
                    class="yk-input"
                    placeholder="Search components…"
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev))
                />
            </div>
            <div class="yk-palette__grid">
                <For
                    each=filtered
                    key=|e| e.label
                    children=move |entry| {
                        let featured = entry.featured;
                        view! {
                            <button
                                class="yk-card"
                                class:yk-card--featured=featured
                                title=entry.hint
                                draggable="true"
                                on:click=move |_| {
                                    let node = node_with_defaults(kind_from_label(entry.label));
                                    let new_id = node.id;
                                    ctx.document.update(|doc| {
                                        doc.insert_node(node);
                                    });
                                    ctx.selected_node.set(Some(new_id));
                                }
                            >
                                <span class="yk-card__icon" aria-hidden="true">
                                    {entry.icon}
                                </span>
                                <span class="yk-card__label">{entry.label}</span>
                                {if featured {
                                    view! {
                                        <span class="yk-card__badge">"Start here"</span>
                                    }
                                    .into_any()
                                } else {
                                    view! { <span/> }.into_any()
                                }}
                            </button>
                        }
                    }
                />
            </div>
        </div>
    }
}

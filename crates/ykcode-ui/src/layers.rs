use leptos::prelude::*;
use ykcode_core::{Document, NodeId, SiblingDirection};

use crate::EditorCtx;

#[derive(Clone)]
struct LayerItem {
    id: NodeId,
    depth: u32,
    name: String,
    icon: &'static str,
}

fn flatten_tree(doc: &Document) -> Vec<LayerItem> {
    let mut items = Vec::new();
    for page in &doc.pages {
        collect_items(doc, page.root_node, 0, &mut items);
    }
    items
}

fn collect_items(doc: &Document, id: NodeId, depth: u32, out: &mut Vec<LayerItem>) {
    if let Some(node) = doc.node(&id) {
        out.push(LayerItem {
            id,
            depth,
            name: node.name.clone(),
            icon: node.kind.icon(),
        });
        for &child in &node.children {
            collect_items(doc, child, depth + 1, out);
        }
    }
}

#[component]
pub(crate) fn LayerTree() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");
    let items = move || ctx.document.with(flatten_tree);

    view! {
        <div class="yk-layers">
            <div class="yk-layers__header">
                {move || ctx.document.with(|d| d.name.clone())}
            </div>
            <For
                each=items
                key=|item| item.id.0.to_string()
                children=move |item| {
                    let id = item.id;
                    let depth = item.depth;
                    let is_selected = move || ctx.selected_node.get() == Some(id);
                    let indent = format!("padding-left:{}rem", depth as f32 * 0.875);

                    let can_up = move || {
                        ctx.document.with(|d| {
                            d.parent_of(id)
                                .and_then(|pid| d.node(&pid))
                                .and_then(|p| p.children.iter().position(|&c| c == id))
                                .map(|i| i > 0)
                                .unwrap_or(false)
                        })
                    };

                    let can_down = move || {
                        ctx.document.with(|d| {
                            d.parent_of(id)
                                .and_then(|pid| d.node(&pid))
                                .map(|p| {
                                    p.children
                                        .iter()
                                        .position(|&c| c == id)
                                        .map(|i| i + 1 < p.children.len())
                                        .unwrap_or(false)
                                })
                                .unwrap_or(false)
                        })
                    };

                    view! {
                        <div class="yk-layer-wrap">
                            <button
                                class="yk-layer-row"
                                class:yk-layer-row--selected=is_selected
                                style=indent
                                on:click=move |_| ctx.selected_node.set(Some(id))
                            >
                                <span class="yk-layer-icon" aria-hidden="true">
                                    {item.icon}
                                </span>
                                <span class="yk-layer-name">{item.name}</span>
                            </button>
                            {move || {
                                if is_selected() {
                                    view! {
                                        <div class="yk-layer-reorder">
                                            <button
                                                class="yk-reorder-btn"
                                                disabled=move || !can_up()
                                                aria-label="Move up"
                                                on:click=move |_| {
                                                    ctx.document.update(|d| {
                                                        let _ = d.move_sibling(id, SiblingDirection::Up);
                                                    });
                                                }
                                            >
                                                "↑"
                                            </button>
                                            <button
                                                class="yk-reorder-btn"
                                                disabled=move || !can_down()
                                                aria-label="Move down"
                                                on:click=move |_| {
                                                    ctx.document.update(|d| {
                                                        let _ =
                                                            d.move_sibling(id, SiblingDirection::Down);
                                                    });
                                                }
                                            >
                                                "↓"
                                            </button>
                                        </div>
                                    }
                                    .into_any()
                                } else {
                                    view! { <span/> }.into_any()
                                }
                            }}
                        </div>
                    }
                }
            />
        </div>
    }
}

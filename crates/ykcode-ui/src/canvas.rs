use leptos::prelude::*;
use ykcode_core::{Document, NodeId, NodeKind};

use crate::dnd::{kind_from_payload, node_with_defaults, MIME_FALLBACK, MIME_KIND};
use crate::node_inline_style;
use crate::EditorCtx;

fn default_node_content(kind: &NodeKind) -> Option<String> {
    match kind {
        NodeKind::Text => Some("Add your text".into()),
        NodeKind::Button => Some("Button".into()),
        NodeKind::Image => Some("🖼 Add image".into()),
        NodeKind::Section
        | NodeKind::Stack
        | NodeKind::Container
        | NodeKind::Divider
        | NodeKind::Spacer => None,
    }
}

#[component]
fn EditableLeafNode(
    node_id: NodeId,
    kind_class: String,
    kind_label: &'static str,
    style: String,
) -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");

    let current_content = move || {
        ctx.document.with(|d| {
            d.node(&node_id)
                .and_then(|n| n.content.clone())
                .unwrap_or_else(|| "...".into())
        })
    };

    let draft = RwSignal::new(String::new());
    let is_editing = move || ctx.editing_node.get() == Some(node_id);
    let is_selected = move || ctx.selected_node.get() == Some(node_id);
    let is_just_dropped = move || ctx.just_dropped.get() == Some(node_id);

    let start_edit = move || {
        draft.set(ctx.document.with(|d| {
            d.node(&node_id)
                .and_then(|n| n.content.clone())
                .unwrap_or_default()
        }));
        ctx.editing_node.set(Some(node_id));
    };

    let commit = move || {
        let text = draft.get();
        ctx.document.update(|d| {
            if let Some(n) = d.nodes.get_mut(&node_id) {
                n.content = Some(text);
            }
        });
        ctx.editing_node.set(None);
    };

    let cancel = move || ctx.editing_node.set(None);

    view! {
        <div
            class=kind_class
            style=style
            class:is-selected=is_selected
            class:is-editing=is_editing
            class:yk-node--just-dropped=is_just_dropped
            data-kind=kind_label
            on:click=move |ev| {
                ev.stop_propagation();
                ctx.selected_node.set(Some(node_id));
                ctx.just_dropped.set(None);
            }
            on:dblclick=move |ev| {
                ev.stop_propagation();
                start_edit();
            }
        >
            {move || {
                if is_editing() {
                    view! {
                        <input
                            type="text"
                            class="yk-inline-edit"
                            prop:value=move || draft.get()
                            on:input=move |ev| draft.set(event_target_value(&ev))
                            on:blur=move |_| commit()
                            on:keydown=move |ev| {
                                match ev.key().as_str() {
                                    "Enter" => {
                                        ev.prevent_default();
                                        commit();
                                    }
                                    "Escape" => {
                                        ev.prevent_default();
                                        cancel();
                                    }
                                    _ => {}
                                }
                            }
                        />
                    }
                    .into_any()
                } else {
                    view! { <span>{current_content()}</span> }.into_any()
                }
            }}
        </div>
    }
}

fn render_node(doc: &Document, id: NodeId, ctx: EditorCtx) -> AnyView {
    let Some(node) = doc.node(&id) else {
        return view! { <span/> }.into_any();
    };

    let kind = node.kind.clone();
    let kind_class = format!("yk-node yk-node--{}", kind.label().to_lowercase());
    let kind_label = kind.label();
    let children_ids = node.children.clone();
    let inline_style = node_inline_style(node);

    if matches!(kind, NodeKind::Text | NodeKind::Button) && children_ids.is_empty() {
        return view! {
            <EditableLeafNode
                node_id=id
                kind_class=kind_class
                kind_label
                style=inline_style
            />
        }
        .into_any();
    }

    let content = node.content.clone().or_else(|| default_node_content(&kind));
    let has_content = content.is_some();

    let child_views: Vec<AnyView> = children_ids
        .iter()
        .map(|&child_id| render_node(doc, child_id, ctx))
        .collect();

    let is_selected = move || ctx.selected_node.get() == Some(id);
    let is_just_dropped = move || ctx.just_dropped.get() == Some(id);

    if has_content && children_ids.is_empty() {
        view! {
            <div
                class=kind_class
                style=inline_style
                class:is-selected=is_selected
                class:yk-node--just-dropped=is_just_dropped
                data-kind=kind_label
                on:click=move |ev| {
                    ev.stop_propagation();
                    ctx.selected_node.set(Some(id));
                    ctx.just_dropped.set(None);
                }
            >
                {content.unwrap_or_default()}
            </div>
        }
        .into_any()
    } else {
        let is_empty = children_ids.is_empty();
        view! {
            <div
                class=kind_class
                style=inline_style
                class:is-selected=is_selected
                class:yk-node--just-dropped=is_just_dropped
                data-kind=kind_label
                data-empty=if is_empty { "true" } else { "false" }
                on:click=move |ev| {
                    ev.stop_propagation();
                    ctx.selected_node.set(Some(id));
                    ctx.just_dropped.set(None);
                }
            >
                {child_views}
            </div>
        }
        .into_any()
    }
}

#[component]
pub(crate) fn CanvasArea() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");
    let zoom_label = move || format!("{}%", ctx.zoom.get() as u32);
    let drag_over = ctx.drag_over_artboard;

    view! {
        <main class="yk-canvas-wrap">
            <div
                class="yk-canvas"
                on:click=move |_| {
                    ctx.selected_node.set(None);
                    ctx.just_dropped.set(None);
                }
            >
                <div
                    class="yk-artboard"
                    class:yk-drop-zone--active=move || drag_over.get()
                    data-mode="edit"
                    on:dragover=move |ev| {
                        ev.prevent_default();
                        if let Some(dt) = ev.data_transfer() {
                            dt.set_drop_effect("copy");
                        }
                        drag_over.set(true);
                    }
                    on:dragleave=move |_| {
                        drag_over.set(false);
                    }
                    on:drop=move |ev| {
                        ev.prevent_default();
                        drag_over.set(false);

                        let kind_label = ev.data_transfer().and_then(|dt| {
                            dt.get_data(MIME_KIND)
                                .ok()
                                .filter(|s| !s.is_empty())
                                .or_else(|| dt.get_data(MIME_FALLBACK).ok())
                                .filter(|s| !s.is_empty())
                        });

                        if let Some(label) = kind_label {
                            let node = node_with_defaults(kind_from_payload(&label));
                            let new_id = node.id;
                            ctx.document.update(|doc| {
                                let parent_id = doc
                                    .active_page_id
                                    .and_then(|pid| doc.pages.iter().find(|p| p.id == pid))
                                    .map(|p| p.root_node);

                                if let Some(parent_id) = parent_id {
                                    let index = doc
                                        .node(&parent_id)
                                        .map(|n| n.children.len())
                                        .unwrap_or(0);
                                    doc.insert_at(node, parent_id, index);
                                }
                            });
                            ctx.selected_node.set(Some(new_id));
                            ctx.just_dropped.set(Some(new_id));
                        }
                    }
                >
                    {move || {
                        ctx.document.with(|d| {
                            let is_empty = d
                                .active_page()
                                .and_then(|p| d.node(&p.root_node))
                                .map(|n| n.children.is_empty())
                                .unwrap_or(true);

                            if is_empty {
                                view! { <EmptyState/> }.into_any()
                            } else if let Some(page) = d.active_page() {
                                let root_id = page.root_node;
                                let children: Vec<_> = d
                                    .node(&root_id)
                                    .map(|n| n.children.clone())
                                    .unwrap_or_default();

                                let views: Vec<AnyView> = children
                                    .iter()
                                    .map(|&child_id| render_node(d, child_id, ctx))
                                    .collect();

                                view! {
                                    <div class="yk-artboard__nodes">{views}</div>
                                }
                                .into_any()
                            } else {
                                view! { <EmptyState/> }.into_any()
                            }
                        })
                    }}
                </div>
            </div>

            <div class="yk-zoom-bar">
                <button
                    class="yk-zoom-btn"
                    aria-label="Zoom out"
                    on:click=move |_| ctx.zoom.update(|z| *z = (*z - 10.0).max(10.0))
                >
                    "−"
                </button>
                <span class="yk-zoom-val">{zoom_label}</span>
                <button
                    class="yk-zoom-btn"
                    aria-label="Zoom in"
                    on:click=move |_| ctx.zoom.update(|z| *z = (*z + 10.0).min(400.0))
                >
                    "+"
                </button>
                <button
                    class="yk-zoom-btn"
                    aria-label="Reset zoom"
                    on:click=move |_| ctx.zoom.set(100.0)
                >
                    "⟳"
                </button>
            </div>
        </main>
    }
}

#[component]
fn EmptyState() -> impl IntoView {
    view! {
        <div class="yk-empty">
            <div class="yk-empty__glyph" aria-hidden="true">"◇"</div>
            <h2 class="yk-empty__title">"Build your first section"</h2>
            <p class="yk-empty__body">"Tap a component or drag it onto this page."</p>
            <div class="yk-empty__actions">
                <button class="yk-btn yk-btn--primary">"Add section"</button>
                <button class="yk-btn yk-btn--ghost">"Show me around"</button>
            </div>
        </div>
    }
}

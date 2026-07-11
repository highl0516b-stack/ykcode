use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use ykcode_core::{Document, Node, NodeId, NodeKind, Size};

// ---------------------------------------------------------------------------
// Editor context — shared reactive state
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct EditorCtx {
    pub left_panel_open: RwSignal<bool>,
    pub right_panel_open: RwSignal<bool>,
    pub active_left_tab: RwSignal<LeftTab>,
    pub zoom: RwSignal<f32>,
    pub selected_node: RwSignal<Option<NodeId>>,
    pub document: RwSignal<Document>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LeftTab {
    Components,
    Layers,
}

// ---------------------------------------------------------------------------
// Shell (SSR only)
// ---------------------------------------------------------------------------

#[cfg(feature = "ssr")]
pub fn shell(options: leptos_config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

// ---------------------------------------------------------------------------
// App — root with meta context and router
// ---------------------------------------------------------------------------

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ykcode.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700&family=Space+Grotesk:wght@400;500;600;700&display=swap"
        />
        <Title text="ykcode — Zero-Code Platform"/>
        <Router>
            <Routes fallback=|| view! { <p class="yk-not-found">"404 – Page not found"</p> }>
                <Route path=StaticSegment("") view=Editor/>
            </Routes>
        </Router>
    }
}

// ---------------------------------------------------------------------------
// Editor shell
// ---------------------------------------------------------------------------

#[component]
fn Editor() -> impl IntoView {
    let ctx = EditorCtx {
        left_panel_open: RwSignal::new(true),
        right_panel_open: RwSignal::new(true),
        active_left_tab: RwSignal::new(LeftTab::Components),
        zoom: RwSignal::new(100.0f32),
        selected_node: RwSignal::new(None),
        document: RwSignal::new(Document::default()),
    };
    provide_context(ctx);

    view! {
        <div class="yk-shell">
            <Toolbar/>
            <div class="yk-workspace">
                <LeftPanel/>
                <CanvasArea/>
                <PropertiesPanel/>
            </div>
            <StatusBar/>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Toolbar
// ---------------------------------------------------------------------------

#[component]
fn Toolbar() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");
    let doc_name = move || ctx.document.with(|d| d.name.clone());

    view! {
        <header class="yk-toolbar">
            <div class="yk-toolbar__start">
                <span class="yk-brand">"yk"</span>
                <div class="yk-toolbar__sep"/>
                <span class="yk-toolbar__doc">{doc_name}</span>
            </div>

            <nav class="yk-toolbar__center" aria-label="Breakpoints">
                <button class="yk-bp yk-bp--on" aria-pressed="true">
                    <span aria-hidden="true">"⬚"</span>
                    "Desktop"
                </button>
                <button class="yk-bp" aria-pressed="false">
                    <span aria-hidden="true">"▭"</span>
                    "Tablet"
                </button>
                <button class="yk-bp" aria-pressed="false">
                    <span aria-hidden="true">"▯"</span>
                    "Mobile"
                </button>
            </nav>

            <div class="yk-toolbar__end">
                <button class="yk-btn yk-btn--ghost" aria-label="Undo">"↩"</button>
                <button class="yk-btn yk-btn--ghost" aria-label="Redo">"↪"</button>
                <div class="yk-toolbar__sep"/>
                <button class="yk-btn yk-btn--secondary">"Preview"</button>
                <button class="yk-btn yk-btn--primary">"Publish"</button>
            </div>
        </header>
    }
}

// ---------------------------------------------------------------------------
// Left panel
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Component palette — click-to-add + drag-ready cards
// ---------------------------------------------------------------------------

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
fn ComponentPalette() -> impl IntoView {
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
                                    ctx.document.update(|doc| {
                                        doc.insert_node(node);
                                    });
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

// ---------------------------------------------------------------------------
// Canvas node renderer (recursive via AnyView)
// ---------------------------------------------------------------------------

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

fn render_node(doc: &Document, id: NodeId, ctx: EditorCtx) -> AnyView {
    let Some(node) = doc.node(&id) else {
        return view! { <span/> }.into_any();
    };

    let kind = node.kind.clone();
    let kind_class = format!("yk-node yk-node--{}", kind.label().to_lowercase());
    let kind_label = kind.label();
    let content = node.content.clone().or_else(|| default_node_content(&kind));
    let children_ids = node.children.clone();
    let has_content = content.is_some();

    let child_views: Vec<AnyView> = children_ids
        .iter()
        .map(|&child_id| render_node(doc, child_id, ctx))
        .collect();

    let is_selected = move || ctx.selected_node.get() == Some(id);

    if has_content && children_ids.is_empty() {
        view! {
            <div
                class=kind_class
                class:is-selected=is_selected
                data-kind=kind_label
                on:click=move |ev| {
                    ev.stop_propagation();
                    ctx.selected_node.set(Some(id));
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
                class:is-selected=is_selected
                data-kind=kind_label
                data-empty=if is_empty { "true" } else { "false" }
                on:click=move |ev| {
                    ev.stop_propagation();
                    ctx.selected_node.set(Some(id));
                }
            >
                {child_views}
            </div>
        }
        .into_any()
    }
}

// ---------------------------------------------------------------------------
// Layer tree (flat pre-order traversal)
// ---------------------------------------------------------------------------

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
fn LayerTree() -> impl IntoView {
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
                    let is_selected = move || ctx.selected_node.get() == Some(id);
                    let indent = format!("padding-left:{}rem", item.depth as f32 * 0.875);
                    view! {
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
                    }
                }
            />
        </div>
    }
}

// ---------------------------------------------------------------------------
// Canvas — infinite dot-grid workspace with artboard
// ---------------------------------------------------------------------------

#[component]
fn CanvasArea() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");
    let zoom_label = move || format!("{}%", ctx.zoom.get() as u32);

    view! {
        <main class="yk-canvas-wrap">
            <div
                class="yk-canvas"
                on:click=move |_| ctx.selected_node.set(None)
            >
                <div class="yk-artboard" data-mode="edit">
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

// ---------------------------------------------------------------------------
// Empty state
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Properties panel
// ---------------------------------------------------------------------------

#[component]
fn PropertiesPanel() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");

    view! {
        <aside
            class="yk-right"
            class:yk-right--closed=move || !ctx.right_panel_open.get()
        >
            <div class="yk-props">
                {move || {
                    if ctx.selected_node.get().is_some() {
                        view! { <NodeInspector/> }.into_any()
                    } else {
                        view! { <PageInspector/> }.into_any()
                    }
                }}
            </div>

            <button
                class="yk-rail-toggle yk-rail-toggle--right"
                aria-label="Toggle properties"
                on:click=move |_| ctx.right_panel_open.update(|v| *v = !*v)
            >
                {move || if ctx.right_panel_open.get() { "›" } else { "‹" }}
            </button>
        </aside>
    }
}

#[component]
fn PageInspector() -> impl IntoView {
    view! {
        <div>
            <div class="yk-props__header">"Page"</div>
            <PropSection title="Canvas">
                <PropRow label="Background" value="#F8F9FC"/>
                <PropRow label="Width" value="1440 px"/>
            </PropSection>
            <PropSection title="Typography">
                <PropRow label="Font" value="Manrope"/>
                <PropRow label="Base size" value="16 px"/>
            </PropSection>
        </div>
    }
}

#[component]
fn NodeInspector() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");

    view! {
        {move || {
            ctx.selected_node
                .get()
                .and_then(|id| {
                    ctx.document.with(|d| {
                        d.node(&id).map(|n| {
                            let name = n.name.clone();
                            let kind = n.kind.label();
                            let display = format!("{:?}", n.layout.display);
                            let direction = format!("{:?}", n.layout.direction);
                            let width = size_label(&n.layout.width);
                            let height = size_label(&n.layout.height);
                            let opacity =
                                format!("{}%", (n.appearance.opacity * 100.0) as u32);
                            let bg = n
                                .appearance
                                .background
                                .as_ref()
                                .map(|c| format!("rgb({} {} {})", c.r, c.g, c.b))
                                .unwrap_or_else(|| "None".into());

                            view! {
                                <div>
                                    <div class="yk-props__header">
                                        <span class="yk-props__badge">{kind}</span>
                                        {name}
                                    </div>
                                    <PropSection title="Layout">
                                        <PropRow label="Display" value=display/>
                                        <PropRow label="Direction" value=direction/>
                                    </PropSection>
                                    <PropSection title="Size">
                                        <PropRow label="Width" value=width/>
                                        <PropRow label="Height" value=height/>
                                    </PropSection>
                                    <PropSection title="Appearance">
                                        <PropRow label="Background" value=bg/>
                                        <PropRow label="Opacity" value=opacity/>
                                    </PropSection>
                                    <div class="yk-props__actions">
                                        <button
                                            class="yk-btn yk-btn--ghost yk-btn--danger"
                                            on:click=move |_| {
                                                if let Some(sel_id) = ctx.selected_node.get() {
                                                    ctx.document.update(|doc| doc.remove_node(sel_id));
                                                    ctx.selected_node.set(None);
                                                }
                                            }
                                        >
                                            "Delete node"
                                        </button>
                                    </div>
                                </div>
                            }
                        })
                    })
                })
        }}
    }
}

fn size_label(size: &Size) -> String {
    match size {
        Size::Auto => "Auto".into(),
        Size::Fill => "Fill".into(),
        Size::Fixed(v) => format!("{v} px"),
        Size::Percent(v) => format!("{v}%"),
    }
}

#[component]
fn PropSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <details class="yk-prop-section" open>
            <summary class="yk-prop-section__title">{title}</summary>
            <div class="yk-prop-rows">{children()}</div>
        </details>
    }
}

#[component]
fn PropRow(label: &'static str, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="yk-prop-row">
            <span class="yk-prop-label">{label}</span>
            <span class="yk-prop-value">{value}</span>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Status bar
// ---------------------------------------------------------------------------

#[component]
fn StatusBar() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");
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
            <span class="yk-status__l">"● Auto-saved"</span>
            <span class="yk-status__r">
                {move || {
                    let count = node_count();
                    if count == 0 {
                        "Empty page".into()
                    } else {
                        format!("{count} component{}", if count == 1 { "" } else { "s" })
                    }
                }}
            </span>
        </footer>
    }
}

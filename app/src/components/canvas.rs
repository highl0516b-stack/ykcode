use leptos::prelude::*;
use shared::{CanvasComponent, ComponentId, ViewportTransform};

#[component]
pub fn CanvasViewport(
    components: Signal<Vec<CanvasComponent>>,
    selected_id: ReadSignal<Option<ComponentId>>,
    set_selected_id: WriteSignal<Option<ComponentId>>,
    transform: ReadSignal<ViewportTransform>,
) -> impl IntoView {
    view! {
        <div class="canvas-viewport" aria-label="Design canvas">
            <div
                class="canvas-surface"
                style=move || {
                    let t = transform.get();
                    format!(
                        "transform: translate({}px, {}px) scale({})",
                        t.translate_x, t.translate_y, t.scale
                    )
                }
            >
                // Canvas grid background is pure CSS via .canvas-surface
                <div class="canvas-artboard">
                    {move || components.get().into_iter().map(|c| {
                        let id = c.id;
                        let is_selected = move || selected_id.get() == Some(id);
                        view! {
                            <ComponentNode
                                component=c
                                is_selected=is_selected
                                on_select=move || set_selected_id.set(Some(id))
                            />
                        }
                    }).collect_view()}
                </div>
                <SnapGuideOverlay/>
                <SelectionOverlay
                    components=components
                    selected_id=selected_id
                    transform=transform
                />
            </div>

            <ZoomControl transform=transform/>
        </div>
    }
}

#[component]
fn ComponentNode(
    component: CanvasComponent,
    is_selected: impl Fn() -> bool + Send + Sync + 'static,
    on_select: impl Fn() + Send + Sync + 'static,
) -> impl IntoView {
    let bounds = component.bounds.clone();
    let kind_label = format!("{:?}", component.kind);
    let name = component.name.clone();
    let opacity = component.style.opacity;
    let rotation = component.rotation;

    view! {
        <div
            class="canvas-node"
            class:selected=is_selected
            style=move || format!(
                "left:{}px; top:{}px; width:{}px; height:{}px; opacity:{}; transform: rotate({}deg);",
                bounds.x, bounds.y, bounds.width, bounds.height,
                opacity, rotation,
            )
            on:click=move |e| {
                e.stop_propagation();
                on_select();
            }
            data-kind=kind_label
        >
            <span class="canvas-node__label">{name}</span>
        </div>
    }
}

#[component]
fn SnapGuideOverlay() -> impl IntoView {
    // Snap guides are rendered imperatively by JS/gesture handlers;
    // this component provides the host element.
    view! {
        <div class="snap-guide-overlay" aria-hidden="true" />
    }
}

#[component]
fn SelectionOverlay(
    components: Signal<Vec<CanvasComponent>>,
    selected_id: ReadSignal<Option<ComponentId>>,
    transform: ReadSignal<ViewportTransform>,
) -> impl IntoView {
    let selected = move || {
        let id = selected_id.get()?;
        components.get().into_iter().find(|c| c.id == id)
    };

    view! {
        {move || selected().map(|c| {
            let scale = transform.get().scale;
            let handle_size = (8.0_f32 / scale).max(6.0);
            view! {
                <div
                    class="selection-frame"
                    style=format!(
                        "left:{}px; top:{}px; width:{}px; height:{}px;",
                        c.bounds.x, c.bounds.y, c.bounds.width, c.bounds.height
                    )
                    aria-hidden="true"
                >
                    // Eight resize handles
                    {corner_positions().into_iter().map(|(cx, cy, cursor)| view! {
                        <div
                            class="selection-handle"
                            style=format!(
                                "left:{}%; top:{}%; width:{}px; height:{}px; cursor:{};",
                                cx, cy, handle_size, handle_size, cursor
                            )
                        />
                    }).collect_view()}
                    // Rotation handle
                    <div class="selection-handle selection-handle--rotate"
                        style=format!("width:{}px; height:{}px;", handle_size + 2.0, handle_size + 2.0)
                    />
                </div>
            }
        })}
    }
}

fn corner_positions() -> Vec<(f32, f32, &'static str)> {
    vec![
        (0.0, 0.0, "nwse-resize"),
        (50.0, 0.0, "ns-resize"),
        (100.0, 0.0, "nesw-resize"),
        (100.0, 50.0, "ew-resize"),
        (100.0, 100.0, "nwse-resize"),
        (50.0, 100.0, "ns-resize"),
        (0.0, 100.0, "nesw-resize"),
        (0.0, 50.0, "ew-resize"),
    ]
}

#[component]
fn ZoomControl(transform: ReadSignal<ViewportTransform>) -> impl IntoView {
    let percent = move || format!("{:.0}%", transform.get().scale * 100.0);

    view! {
        <div class="zoom-control glass-canvas-overlay">
            <button class="zoom-control__btn" aria-label="Zoom out">"-"</button>
            <span class="zoom-control__value">{percent}</span>
            <button class="zoom-control__btn" aria-label="Zoom in">"+"</button>
        </div>
    }
}

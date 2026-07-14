use leptos::prelude::*;
use ykcode_core::FlexDirection;

use crate::{with_history, EditorCtx};

#[component]
pub(crate) fn PropertiesPanel() -> impl IntoView {
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

    let gap_val = RwSignal::new(0.0f32);
    let opacity_val = RwSignal::new(1.0f32);

    Effect::new(move |_| {
        if let Some(id) = ctx.selected_node.get() {
            gap_val.set(
                ctx.document
                    .with(|d| d.node(&id).map(|n| n.layout.gap).unwrap_or(0.0)),
            );
            opacity_val.set(
                ctx.document
                    .with(|d| d.node(&id).map(|n| n.appearance.opacity).unwrap_or(1.0)),
            );
        }
    });

    let direction_str = move || {
        ctx.selected_node
            .get()
            .and_then(|id| {
                ctx.document
                    .with(|d| d.node(&id).map(|n| format!("{:?}", n.layout.direction)))
            })
            .unwrap_or_default()
    };

    let set_direction = move |dir: FlexDirection| {
        if let Some(id) = ctx.selected_node.get() {
            with_history(ctx, |d| {
                if let Some(n) = d.nodes.get_mut(&id) {
                    n.layout.direction = dir;
                }
            });
        }
    };

    view! {
        {move || {
            ctx.selected_node
                .get()
                .and_then(|id| {
                    ctx.document.with(|d| {
                        d.node(&id).map(|n| {
                            let name = n.name.clone();
                            let kind = n.kind.label();
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
                                        <div class="yk-prop-row">
                                            <span class="yk-prop-label">"Direction"</span>
                                            <div class="yk-direction-toggle" role="group" aria-label="Direction">
                                                <button
                                                    class="yk-dir-btn"
                                                    class:yk-dir-btn--on=move || direction_str() == "Column"
                                                    aria-pressed=move || direction_str() == "Column"
                                                    on:click=move |_| set_direction(FlexDirection::Column)
                                                >
                                                    "↕ Col"
                                                </button>
                                                <button
                                                    class="yk-dir-btn"
                                                    class:yk-dir-btn--on=move || direction_str() == "Row"
                                                    aria-pressed=move || direction_str() == "Row"
                                                    on:click=move |_| set_direction(FlexDirection::Row)
                                                >
                                                    "↔ Row"
                                                </button>
                                            </div>
                                        </div>
                                        <div class="yk-prop-row">
                                            <span class="yk-prop-label">"Gap"</span>
                                            <div class="yk-scrub-field">
                                                <input
                                                    type="number"
                                                    class="yk-scrub-input"
                                                    min="0"
                                                    max="500"
                                                    step="1"
                                                    prop:value=move || gap_val.get().to_string()
                                                    on:input=move |ev| {
                                                        let Ok(v) = event_target_value(&ev).parse::<f32>() else {
                                                            return;
                                                        };
                                                        let v = v.clamp(0.0, 500.0);
                                                        gap_val.set(v);
                                                        if let Some(sel_id) = ctx.selected_node.get() {
                                                            with_history(ctx, |d| {
                                                                if let Some(node) = d.nodes.get_mut(&sel_id) {
                                                                    node.layout.gap = v;
                                                                }
                                                            });
                                                        }
                                                    }
                                                />
                                                <span class="yk-scrub-unit">"px"</span>
                                            </div>
                                        </div>
                                    </PropSection>
                                    <PropSection title="Appearance">
                                        <PropRow label="Background" value=bg/>
                                        <div class="yk-prop-row">
                                            <span class="yk-prop-label">"Opacity"</span>
                                            <div class="yk-opacity-ctrl">
                                                <input
                                                    type="range"
                                                    class="yk-opacity-range"
                                                    min="0"
                                                    max="100"
                                                    step="1"
                                                    prop:value=move || (opacity_val.get() * 100.0) as u32
                                                    on:input=move |ev| {
                                                        let Ok(v) = event_target_value(&ev).parse::<f32>() else {
                                                            return;
                                                        };
                                                        let opacity = (v / 100.0).clamp(0.0, 1.0);
                                                        opacity_val.set(opacity);
                                                        if let Some(sel_id) = ctx.selected_node.get() {
                                                            with_history(ctx, |d| {
                                                                if let Some(node) = d.nodes.get_mut(&sel_id) {
                                                                    node.appearance.opacity = opacity;
                                                                }
                                                            });
                                                        }
                                                    }
                                                />
                                                <span class="yk-opacity-val">
                                                    {move || format!("{}%", (opacity_val.get() * 100.0) as u32)}
                                                </span>
                                            </div>
                                        </div>
                                    </PropSection>
                                    <div class="yk-props__actions">
                                        <button
                                            class="yk-btn yk-btn--ghost yk-btn--danger"
                                            on:click=move |_| {
                                                if let Some(sel_id) = ctx.selected_node.get() {
                                                    with_history(ctx, |doc| doc.remove_node(sel_id));
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

use leptos::prelude::*;
use shared::{CanvasComponent, ComponentId, ProjectPalette};

#[component]
pub fn PropertiesPanel(
    components: Signal<Vec<CanvasComponent>>,
    selected_id: ReadSignal<Option<ComponentId>>,
    palette: Signal<ProjectPalette>,
) -> impl IntoView {
    let selected = move || {
        let id = selected_id.get()?;
        components.get().into_iter().find(|c| c.id == id)
    };

    view! {
        <aside class="properties-panel glass-panel" aria-label="Properties">
            {move || match selected() {
                None => view! {
                    <div class="properties-panel__empty">
                        <p>"Select a component to edit its properties."</p>
                    </div>
                }
                .into_any(),
                Some(c) => view! {
                    <div class="properties-panel__content">
                        // Colour section is always first per UX spec
                        <ColorSection palette=palette component=c.clone()/>
                        <LayoutSection component=c.clone()/>
                        <SizeSection component=c.clone()/>
                    </div>
                }
                .into_any(),
            }}
        </aside>
    }
}

/// Collapsible section header.
/// Callers must pass an `RwSignal<bool>` and gate their body with `<Show>`.
#[component]
fn PanelHeader(title: &'static str, expanded: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="panel-header">
            <button
                class="panel-header__toggle"
                on:click=move |_| expanded.update(|v| *v = !*v)
                aria-expanded=move || expanded.get().to_string()
            >
                <svg
                    class="panel-header__chevron"
                    class:rotated=move || !expanded.get()
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.75"
                >
                    <polyline points="6 9 12 15 18 9"/>
                </svg>
                <span class="panel-header__title">{title}</span>
            </button>
        </div>
    }
}

#[component]
fn ColorSection(palette: Signal<ProjectPalette>, component: CanvasComponent) -> impl IntoView {
    // Store strings in StoredValue so Show children are Fn, not FnOnce.
    let current_bg = StoredValue::new(component.style.background.clone().unwrap_or_default());
    let expanded = RwSignal::new(true);

    view! {
        <section class="props-section" aria-label="Color properties">
            <PanelHeader title="Color" expanded=expanded/>
            <Show when=move || expanded.get()>
                <div class="props-section__body">
                    <div class="props-row">
                        <label class="props-label">"Background"</label>
                        <div class="color-swatch-row">
                            <div
                                class="color-swatch"
                                style=move || format!("background:{}", current_bg.get_value())
                                title=move || current_bg.get_value()
                            />
                            <span class="props-value">{move || current_bg.get_value()}</span>
                        </div>
                    </div>
                    <div class="palette-swatches">
                        <PaletteSwatch color=move || palette.get().primary   label="Primary"/>
                        <PaletteSwatch color=move || palette.get().secondary label="Secondary"/>
                        <PaletteSwatch color=move || palette.get().accent    label="Accent"/>
                        <PaletteSwatch color=move || palette.get().success   label="Success"/>
                        <PaletteSwatch color=move || palette.get().warning   label="Warning"/>
                        <PaletteSwatch color=move || palette.get().error     label="Error"/>
                    </div>
                </div>
            </Show>
        </section>
    }
}

#[component]
fn PaletteSwatch(
    color: impl Fn() -> String + Send + Sync + 'static,
    label: &'static str,
) -> impl IntoView {
    view! {
        <button
            class="palette-swatch"
            style=move || format!("background:{}", color())
            title=label
            aria-label=label
        />
    }
}

#[component]
fn LayoutSection(component: CanvasComponent) -> impl IntoView {
    let expanded = RwSignal::new(true);
    let pos_x = StoredValue::new(component.bounds.x.to_string());
    let pos_y = StoredValue::new(component.bounds.y.to_string());

    view! {
        <section class="props-section" aria-label="Layout properties">
            <PanelHeader title="Layout" expanded=expanded/>
            <Show when=move || expanded.get()>
                <div class="props-section__body">
                    <div class="props-row">
                        <label class="props-label">"Position"</label>
                        <div class="props-xy">
                            <PropInput label="X" value=move || pos_x.get_value()/>
                            <PropInput label="Y" value=move || pos_y.get_value()/>
                        </div>
                    </div>
                </div>
            </Show>
        </section>
    }
}

#[component]
fn SizeSection(component: CanvasComponent) -> impl IntoView {
    let expanded = RwSignal::new(true);
    let width = StoredValue::new(component.bounds.width.to_string());
    let height = StoredValue::new(component.bounds.height.to_string());
    let rotation = StoredValue::new(component.rotation.to_string());
    let opacity = StoredValue::new(component.style.opacity.to_string());

    view! {
        <section class="props-section" aria-label="Size and spacing">
            <PanelHeader title="Size" expanded=expanded/>
            <Show when=move || expanded.get()>
                <div class="props-section__body">
                    <div class="props-row">
                        <label class="props-label">"Dimensions"</label>
                        <div class="props-xy">
                            <PropInput label="W" value=move || width.get_value()/>
                            <PropInput label="H" value=move || height.get_value()/>
                        </div>
                    </div>
                    <div class="props-row">
                        <label class="props-label">"Rotation"</label>
                        <PropInput label="°" value=move || rotation.get_value()/>
                    </div>
                    <div class="props-row">
                        <label class="props-label">"Opacity"</label>
                        <input
                            class="props-slider"
                            type="range"
                            min="0"
                            max="1"
                            step="0.01"
                            prop:value=move || opacity.get_value()
                            aria-label="Opacity"
                        />
                    </div>
                </div>
            </Show>
        </section>
    }
}

#[component]
fn PropInput(
    label: &'static str,
    value: impl Fn() -> String + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <div class="prop-input-group">
            <span class="prop-input-group__label">{label}</span>
            <input
                class="prop-input"
                type="number"
                prop:value=value
                aria-label=label
            />
        </div>
    }
}

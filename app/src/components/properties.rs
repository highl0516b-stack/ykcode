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
                }.into_any(),
                Some(c) => view! {
                    <div class="properties-panel__content">
                        <PanelHeader title="Properties"/>

                        // Color section (always first per UX spec)
                        <ColorSection palette=palette component=c.clone()/>

                        // Layout section
                        <LayoutSection component=c.clone()/>

                        // Size & Spacing
                        <SizeSection component=c.clone()/>
                    </div>
                }.into_any()
            }}
        </aside>
    }
}

#[component]
fn PanelHeader(title: &'static str) -> impl IntoView {
    let expanded = RwSignal::new(true);

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
                    width="14" height="14" viewBox="0 0 24 24"
                    fill="none" stroke="currentColor" stroke-width="1.75"
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
    let current_bg = component.style.background.clone().unwrap_or_default();

    view! {
        <section class="props-section" aria-label="Color properties">
            <PanelHeader title="Color"/>
            <div class="props-section__body">
                <div class="props-row">
                    <label class="props-label">"Background"</label>
                    <div class="color-swatch-row">
                        <div
                            class="color-swatch"
                            style=format!("background:{}", current_bg)
                            title=current_bg.clone()
                        />
                        <span class="props-value">{current_bg}</span>
                    </div>
                </div>
                <div class="palette-swatches">
                    <PaletteSwatch color=move || palette.get().primary  label="Primary"/>
                    <PaletteSwatch color=move || palette.get().secondary label="Secondary"/>
                    <PaletteSwatch color=move || palette.get().accent    label="Accent"/>
                    <PaletteSwatch color=move || palette.get().success   label="Success"/>
                    <PaletteSwatch color=move || palette.get().warning   label="Warning"/>
                    <PaletteSwatch color=move || palette.get().error     label="Error"/>
                </div>
            </div>
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
    view! {
        <section class="props-section" aria-label="Layout properties">
            <PanelHeader title="Layout"/>
            <div class="props-section__body">
                <div class="props-row">
                    <label class="props-label">"Position"</label>
                    <div class="props-xy">
                        <PropInput label="X" value=component.bounds.x.to_string()/>
                        <PropInput label="Y" value=component.bounds.y.to_string()/>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SizeSection(component: CanvasComponent) -> impl IntoView {
    view! {
        <section class="props-section" aria-label="Size and spacing">
            <PanelHeader title="Size"/>
            <div class="props-section__body">
                <div class="props-row">
                    <label class="props-label">"Dimensions"</label>
                    <div class="props-xy">
                        <PropInput label="W" value=component.bounds.width.to_string()/>
                        <PropInput label="H" value=component.bounds.height.to_string()/>
                    </div>
                </div>
                <div class="props-row">
                    <label class="props-label">"Rotation"</label>
                    <PropInput label="°" value=component.rotation.to_string()/>
                </div>
                <div class="props-row">
                    <label class="props-label">"Opacity"</label>
                    <input
                        class="props-slider"
                        type="range"
                        min="0" max="1" step="0.01"
                        prop:value=component.style.opacity.to_string()
                        aria-label="Opacity"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn PropInput(label: &'static str, value: String) -> impl IntoView {
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

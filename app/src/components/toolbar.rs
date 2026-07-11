use leptos::prelude::*;

/// Mobile-first floating action toolbar, positioned in the thumb zone.
#[component]
pub fn MobileActionToolbar(
    on_undo: Callback<()>,
    on_redo: Callback<()>,
    on_open_palette: Callback<()>,
    on_open_layers: Callback<()>,
    on_open_properties: Callback<()>,
) -> impl IntoView {
    view! {
        <nav class="mobile-toolbar glass-toolbar" aria-label="Primary actions">
            <button
                class="mobile-toolbar__btn"
                on:click=move |_| on_undo.run(())
                title="Undo"
                aria-label="Undo"
            >
                <UndoIcon/>
            </button>

            <button
                class="mobile-toolbar__btn mobile-toolbar__btn--secondary"
                on:click=move |_| on_open_layers.run(())
                title="Layers"
                aria-label="Open layers panel"
            >
                <LayersIcon/>
            </button>

            // Primary CTA — component palette
            <button
                class="mobile-toolbar__btn mobile-toolbar__btn--primary"
                on:click=move |_| on_open_palette.run(())
                title="Components"
                aria-label="Open component palette"
            >
                <PlusIcon/>
            </button>

            <button
                class="mobile-toolbar__btn mobile-toolbar__btn--secondary"
                on:click=move |_| on_open_properties.run(())
                title="Properties"
                aria-label="Open properties panel"
            >
                <SlidersIcon/>
            </button>

            <button
                class="mobile-toolbar__btn"
                on:click=move |_| on_redo.run(())
                title="Redo"
                aria-label="Redo"
            >
                <RedoIcon/>
            </button>
        </nav>
    }
}

// ── Inline SVG icons (Lucide style, 18px, stroke 1.75) ───────────────────────

#[component]
fn UndoIcon() -> impl IntoView {
    view! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M3 7v6h6"/>
            <path d="M21 17a9 9 0 00-9-9 9 9 0 00-6 2.3L3 13"/>
        </svg>
    }
}

#[component]
fn RedoIcon() -> impl IntoView {
    view! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M21 7v6h-6"/>
            <path d="M3 17a9 9 0 019-9 9 9 0 016 2.3l3 2.7"/>
        </svg>
    }
}

#[component]
fn PlusIcon() -> impl IntoView {
    view! {
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M12 5v14M5 12h14"/>
        </svg>
    }
}

#[component]
fn LayersIcon() -> impl IntoView {
    view! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <polygon points="12 2 22 8.5 12 15 2 8.5 12 2"/>
            <line x1="2" y1="15.5" x2="12" y2="22"/>
            <line x1="22" y1="15.5" x2="12" y2="22"/>
        </svg>
    }
}

#[component]
fn SlidersIcon() -> impl IntoView {
    view! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <line x1="4" y1="21" x2="4" y2="14"/>
            <line x1="4" y1="10" x2="4" y2="3"/>
            <line x1="12" y1="21" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12" y2="3"/>
            <line x1="20" y1="21" x2="20" y2="16"/>
            <line x1="20" y1="12" x2="20" y2="3"/>
            <line x1="1" y1="14" x2="7" y2="14"/>
            <line x1="9" y1="8" x2="15" y2="8"/>
            <line x1="17" y1="16" x2="23" y2="16"/>
        </svg>
    }
}

use leptos::prelude::*;
use shared::DisclosureLevel;

#[component]
pub fn TopCommandBar(
    project_name: Signal<String>,
    disclosure_level: ReadSignal<DisclosureLevel>,
    set_disclosure_level: WriteSignal<DisclosureLevel>,
) -> impl IntoView {
    view! {
        <header class="topbar glass-toolbar">
            <div class="topbar__brand">
                <div class="topbar__logo" aria-label="ykcode">
                    <span class="topbar__logo-mark">"yk"</span>
                    <span class="topbar__logo-code">"code"</span>
                </div>
                <div class="topbar__divider" />
                <span class="topbar__project-name">{project_name}</span>
            </div>

            <div class="topbar__center">
                <button class="topbar__btn" title="Preview">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
                        <polygon points="5 3 19 12 5 21 5 3"/>
                    </svg>
                    <span>"Preview"</span>
                </button>
            </div>

            <div class="topbar__actions">
                <DisclosurePicker
                    current=disclosure_level
                    set_current=set_disclosure_level
                />
                <button class="topbar__btn topbar__btn--primary" title="Publish">
                    "Publish"
                </button>
            </div>
        </header>
    }
}

#[component]
fn DisclosurePicker(
    current: ReadSignal<DisclosureLevel>,
    set_current: WriteSignal<DisclosureLevel>,
) -> impl IntoView {
    let options = [
        (DisclosureLevel::Guided, "Guided"),
        (DisclosureLevel::Standard, "Standard"),
        (DisclosureLevel::Expert, "Expert"),
    ];

    view! {
        <div class="disclosure-picker">
            {options.into_iter().map(|(level, label)| {
                let is_active = move || current.get() == level;
                view! {
                    <button
                        class="disclosure-picker__opt"
                        class:active=is_active
                        on:click=move |_| set_current.set(level)
                        title=label
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

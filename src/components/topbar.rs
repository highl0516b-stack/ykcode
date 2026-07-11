use crate::state::{ActiveTool, DeviceMode, EditorState};
use leptos::prelude::*;

#[component]
pub fn TopBar(state: EditorState) -> impl IntoView {
    let can_undo = state.can_undo();
    let can_redo = state.can_redo();
    let zoom_pct = state.zoom_percent();
    let device = state.device_mode;
    let tool = state.active_tool;
    let theme = state.theme;

    let state_undo = state;
    let state_redo = state;
    let state_theme = state;
    let state_tool_select = state;
    let state_tool_hand = state;

    view! {
        <header class="topbar">
            // Left group
            <div class="topbar__left">
                <div class="topbar__logo">
                    <svg
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M12 2L2 7l10 5 10-5-10-5z" />
                        <path d="M2 17l10 5 10-5" />
                        <path d="M2 12l10 5 10-5" />
                    </svg>
                    <span>"ZeroCo"</span>
                </div>
                <div class="topbar__divider" />
                // Undo
                <button
                    class="btn-icon"
                    disabled=move || !can_undo.get()
                    title="Undo (Ctrl+Z)"
                    on:click=move |_| state_undo.undo()
                >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M9 14L4 9l5-5" />
                        <path d="M4 9h11a6 6 0 0 1 0 12h-1" />
                    </svg>
                </button>
                // Redo
                <button
                    class="btn-icon"
                    disabled=move || !can_redo.get()
                    title="Redo (Ctrl+Shift+Z)"
                    on:click=move |_| state_redo.redo()
                >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M15 14l5-5-5-5" />
                        <path d="M20 9H9a6 6 0 0 0 0 12h1" />
                    </svg>
                </button>
            </div>

            // Center group
            <div class="topbar__center">
                // Select tool
                <button
                    class=move || {
                        if tool.get() == ActiveTool::Select {
                            "btn-icon active"
                        } else {
                            "btn-icon"
                        }
                    }
                    title="Select (V)"
                    on:click=move |_| state_tool_select.active_tool.set(ActiveTool::Select)
                >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M4 4l7 18 3-7 7-3z" />
                    </svg>
                </button>
                // Hand tool
                <button
                    class=move || {
                        if tool.get() == ActiveTool::Hand { "btn-icon active" } else { "btn-icon" }
                    }
                    title="Hand / Pan (H)"
                    on:click=move |_| state_tool_hand.active_tool.set(ActiveTool::Hand)
                >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M18 11V6a2 2 0 0 0-2-2 2 2 0 0 0-2 2" />
                        <path d="M14 10V4a2 2 0 0 0-2-2 2 2 0 0 0-2 2v2" />
                        <path d="M10 10.5V6a2 2 0 0 0-2-2 2 2 0 0 0-2 2v8" />
                        <path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15" />
                    </svg>
                </button>

                <div class="topbar__divider" />

                // Device preview toggle
                <div class="device-toggle">
                    <button
                        class=move || {
                            if device.get() == DeviceMode::Mobile {
                                "device-toggle__btn active"
                            } else {
                                "device-toggle__btn"
                            }
                        }
                        title="Mobile preview"
                        on:click=move |_| device.set(DeviceMode::Mobile)
                    >
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="7" y="2" width="10" height="20" rx="2" />
                            <line x1="12" y1="18" x2="12.01" y2="18" />
                        </svg>
                        "Mobile"
                    </button>
                    <button
                        class=move || {
                            if device.get() == DeviceMode::Tablet {
                                "device-toggle__btn active"
                            } else {
                                "device-toggle__btn"
                            }
                        }
                        title="Tablet preview"
                        on:click=move |_| device.set(DeviceMode::Tablet)
                    >
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="4" y="2" width="16" height="20" rx="2" />
                            <line x1="12" y1="18" x2="12.01" y2="18" />
                        </svg>
                        "Tablet"
                    </button>
                    <button
                        class=move || {
                            if device.get() == DeviceMode::Desktop {
                                "device-toggle__btn active"
                            } else {
                                "device-toggle__btn"
                            }
                        }
                        title="Desktop preview"
                        on:click=move |_| device.set(DeviceMode::Desktop)
                    >
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="2" y="3" width="20" height="14" rx="2" />
                            <line x1="8" y1="21" x2="16" y2="21" />
                            <line x1="12" y1="17" x2="12" y2="21" />
                        </svg>
                        "Desktop"
                    </button>
                </div>

                <div class="topbar__divider" />

                // Zoom display
                <div class="zoom-control" title="Click to reset zoom">
                    <svg
                        width="14"
                        height="14"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <circle cx="11" cy="11" r="8" />
                        <path d="M21 21l-4.35-4.35" />
                    </svg>
                    {move || format!("{}%", zoom_pct.get())}
                </div>
            </div>

            // Right group
            <div class="topbar__right">
                // Theme toggle
                <button
                    class="btn-icon"
                    title="Toggle dark/light mode"
                    on:click=move |_| state_theme.toggle_theme()
                >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        {move || {
                            if theme.get() == "dark" {
                                view! {
                                    <circle cx="12" cy="12" r="5" />
                                    <line x1="12" y1="1" x2="12" y2="3" />
                                    <line x1="12" y1="21" x2="12" y2="23" />
                                    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
                                    <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
                                    <line x1="1" y1="12" x2="3" y2="12" />
                                    <line x1="21" y1="12" x2="23" y2="12" />
                                    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
                                    <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
                                }
                                    .into_any()
                            }
                        }}
                    </svg>
                </button>

                // Preview button
                <button class="btn-icon" title="Preview">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polygon points="5 3 19 12 5 21 5 3" />
                    </svg>
                </button>

                // Publish button
                <button class="btn-primary" title="Publish your project">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                        <path d="M12 2L2 7l10 5 10-5-10-5z" />
                        <path d="M2 17l10 5 10-5" />
                        <path d="M2 12l10 5 10-5" />
                    </svg>
                    "Publish"
                </button>
            </div>
        </header>
    }
}

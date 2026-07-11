use leptos::prelude::*;

use crate::{can_redo, can_undo, redo, undo, with_history, EditorCtx};

#[component]
pub(crate) fn Toolbar() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");
    let is_editing_name = RwSignal::new(false);
    let draft_name = RwSignal::new(String::new());

    let start_rename = move |_| {
        draft_name.set(ctx.document.with(|d| d.name.clone()));
        is_editing_name.set(true);
    };

    let commit_rename = move |_| {
        let name = draft_name.get().trim().to_string();
        if !name.is_empty() {
            with_history(ctx, |doc| doc.name = name);
        }
        is_editing_name.set(false);
    };

    let cancel_rename = move |_| {
        is_editing_name.set(false);
    };

    view! {
        <header class="yk-toolbar">
            <div class="yk-toolbar__start">
                <span class="yk-brand">"yk"</span>
                <div class="yk-toolbar__sep"/>
                {move || {
                    if is_editing_name.get() {
                        view! {
                            <input
                                class="yk-toolbar__doc-input"
                                type="text"
                                prop:value=move || draft_name.get()
                                on:input=move |ev| draft_name.set(event_target_value(&ev))
                                on:blur=move |_| commit_rename(())
                                on:keydown=move |ev| {
                                    match ev.key().as_str() {
                                        "Enter" => {
                                            ev.prevent_default();
                                            commit_rename(());
                                        }
                                        "Escape" => {
                                            ev.prevent_default();
                                            cancel_rename(());
                                        }
                                        _ => {}
                                    }
                                }
                                autofocus
                            />
                        }
                        .into_any()
                    } else {
                        view! {
                            <button
                                class="yk-toolbar__doc"
                                on:click=start_rename
                            >
                                {move || ctx.document.with(|d| d.name.clone())}
                            </button>
                        }
                        .into_any()
                    }
                }}
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
                <button
                    class="yk-btn yk-btn--ghost"
                    aria-label="Undo (Ctrl+Z)"
                    aria-keyshortcuts="Control+Z"
                    disabled=move || !can_undo(ctx)()
                    on:click=move |_| undo(ctx)
                >
                    "↩"
                </button>
                <button
                    class="yk-btn yk-btn--ghost"
                    aria-label="Redo (Ctrl+Shift+Z)"
                    aria-keyshortcuts="Control+Shift+Z"
                    disabled=move || !can_redo(ctx)()
                    on:click=move |_| redo(ctx)
                >
                    "↪"
                </button>
                <div class="yk-toolbar__sep"/>
                <button class="yk-btn yk-btn--secondary">"Preview"</button>
                <button class="yk-btn yk-btn--primary">"Publish"</button>
            </div>
        </header>
    }
}

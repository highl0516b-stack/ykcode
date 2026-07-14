use leptos::prelude::*;

use crate::{with_history, EditorCtx};

#[component]
pub(crate) fn PageStrip() -> impl IntoView {
    let ctx = use_context::<EditorCtx>().expect("EditorCtx missing");

    let pages = move || ctx.document.with(|d| d.pages.clone());
    let active_id = move || ctx.document.with(|d| d.active_page_id);

    let add_page = move |_| {
        with_history(ctx, |doc| {
            let id = doc.add_page(format!("Page {}", doc.pages.len() + 1));
            doc.set_active_page(id);
        });
        ctx.selected_node.set(None);
    };

    view! {
        <nav class="yk-page-strip" aria-label="Pages">
            <div class="yk-page-tabs" role="tablist">
                <For
                    each=pages
                    key=|p| p.id
                    children=move |page| {
                        let page_id = page.id;
                        let is_active = move || active_id() == Some(page_id);
                        let can_delete = move || ctx.document.with(|d| d.pages.len() > 1);

                        view! {
                            <div
                                class="yk-page-tab"
                                data-active=move || is_active().to_string()
                                role="presentation"
                            >
                                <button
                                    class="yk-page-tab__btn"
                                    role="tab"
                                    aria-selected=move || is_active().to_string()
                                    on:click=move |_| {
                                        with_history(ctx, |doc| doc.set_active_page(page_id));
                                        ctx.selected_node.set(None);
                                    }
                                >
                                    {page.name.clone()}
                                </button>
                                {move || if can_delete() {
                                    view! {
                                        <button
                                            class="yk-page-tab__del"
                                            aria-label="Delete page"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                with_history(ctx, |doc| {
                                                    let _ = doc.remove_page(page_id);
                                                });
                                                ctx.selected_node.set(None);
                                            }
                                        >
                                            "×"
                                        </button>
                                    }.into_any()
                                } else {
                                    view! { <span/> }.into_any()
                                }}
                            </div>
                        }
                    }
                />
            </div>
            <button class="yk-page-add" aria-label="Add page" on:click=add_page>
                "+"
            </button>
        </nav>
    }
}

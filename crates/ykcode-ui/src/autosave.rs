#[cfg(feature = "hydrate")]
mod inner {
    use gloo_net::http::Request;
    use leptos::prelude::*;
    use std::{cell::Cell, rc::Rc};
    use ykcode_core::Document;

    use crate::{EditorCtx, SaveStatus};

    pub fn provide_autosave(ctx: EditorCtx) {
        // Skip first run flag (don't save on initial mount)
        let initialized = Rc::new(Cell::new(false));

        Effect::new(move |_| {
            let doc = ctx.document.get(); // reactive subscription

            if !initialized.get() {
                initialized.set(true);
                return;
            }

            let save_status = ctx.save_status;
            save_status.set(SaveStatus::Unsaved);

            let doc_clone = doc.clone();
            leptos::task::spawn_local(async move {
                save_status.set(SaveStatus::Saving);
                match save_doc_http(&doc_clone).await {
                    Ok(()) => save_status.set(SaveStatus::Saved),
                    Err(e) => save_status.set(SaveStatus::Error(e)),
                }
            });
        });
    }

    async fn save_doc_http(doc: &Document) -> Result<(), String> {
        let url = format!("/api/documents/{}", doc.id);
        let body = serde_json::to_string(doc).map_err(|e| e.to_string())?;
        let resp = Request::put(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if resp.ok() {
            Ok(())
        } else {
            Err(format!("HTTP {}", resp.status()))
        }
    }
}

#[cfg(feature = "hydrate")]
pub use inner::provide_autosave;

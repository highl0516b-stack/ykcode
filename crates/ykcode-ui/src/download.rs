#[cfg(feature = "hydrate")]
mod inner {
    use wasm_bindgen::{prelude::*, JsCast};
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};
    use ykcode_core::Document;
    use ykcode_export::export_document;

    pub fn trigger_html_download(doc: &Document) -> Result<(), JsValue> {
        let out = export_document(doc);

        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("no document"))?;

        let parts = js_sys::Array::new();
        parts.push(&JsValue::from_str(&out.html));

        let bag = BlobPropertyBag::new();
        bag.set_type("text/html;charset=utf-8");
        let blob = Blob::new_with_str_sequence_and_options(&parts, &bag)?;
        let url = Url::create_object_url_with_blob(&blob)?;

        let anchor = document
            .create_element("a")?
            .dyn_into::<HtmlAnchorElement>()?;
        anchor.set_href(&url);
        anchor.set_download(&format!("{}.html", slugify(&doc.name)));
        anchor.style().set_property("display", "none")?;
        document
            .body()
            .ok_or_else(|| JsValue::from_str("no body"))?
            .append_child(&anchor)?;
        anchor.click();
        anchor.remove();
        Url::revoke_object_url(&url)?;
        Ok(())
    }

    fn slugify(s: &str) -> String {
        let slug: String = s
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect();
        let trimmed = slug.trim_matches('-');
        if trimmed.is_empty() {
            "export".into()
        } else {
            trimmed.into()
        }
    }
}

#[cfg(feature = "hydrate")]
pub use inner::trigger_html_download;

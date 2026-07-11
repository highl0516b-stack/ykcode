use wasm_bindgen::prelude::*;
use ykcode_ui::App;

#[wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(App);
}

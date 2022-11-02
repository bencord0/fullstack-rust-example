use wasm_bindgen::prelude::*;

#[path = "components/mod.rs"]
pub mod components;
#[path = "model.rs"]
mod model;

pub use model::Model;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    yew::Renderer::<model::Model>::new().render();

    Ok(())
}

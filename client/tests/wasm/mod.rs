use std::time::Duration;
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::{
    Renderer,
    platform::time::sleep,
};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn wasm_basic() {
    assert!(true);
}

fn get_output_element() -> Option<web_sys::Element> {
    let window = web_sys::window()?;
    let document = window.document()?;
    document.get_element_by_id("output")
}

#[test]
async fn render_new_application() -> Result<(), wasm_bindgen::JsValue> {
    // Get an element to render onto
    let element = get_output_element().ok_or("initial root element")?;

    // Render and wait
    Renderer::<client::components::NewApplication>::with_root(element).render();
    sleep(Duration::from_millis(100)).await;

    // Refresh our view of the output
    let element = get_output_element().ok_or("rendered output")?;
    assert_eq!(element.inner_html(), r#"<form><label>App Name: <input type="text"></label></form>"#);

    Ok(())
}

#[test]
async fn render_list_application() -> Result<(), wasm_bindgen::JsValue> {
    // Get an element to render onto
    let element = get_output_element().ok_or("initial root element")?;

    // Render and wait
    Renderer::<client::components::ListApplication>::with_root(element).render();
    sleep(Duration::from_millis(100)).await;

    // Refresh our view of the output
    let element = get_output_element().ok_or("rendered output")?;
    assert_eq!(element.inner_html(), r#""#);

    Ok(())
}

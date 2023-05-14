use wasm_bindgen::prelude::*;
use web_sys::console;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {

    console::log_1(&"OH WOW, HI THERE!".into());

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p").expect("creating new element should succeed");
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val).expect("appending item to doc should succeed");
    
    a + b
}
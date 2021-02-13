use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(message: &str);
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    alert("Hello Fullstack!");

    Ok(())
}

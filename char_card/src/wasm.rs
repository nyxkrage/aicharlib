use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn char_card_from_json(json_str: &str) -> Result<JsValue, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::to_value::<super::Character>(
        &super::Character::from_json(json_str).unwrap(),
    )
}

#[wasm_bindgen]
pub fn char_card_from_png(png_data: Uint8Array) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let png = png_data.to_vec();
    serde_wasm_bindgen::to_value::<super::Character>(&super::Character::from_png(&png[..]).unwrap())
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    Ok(())
}

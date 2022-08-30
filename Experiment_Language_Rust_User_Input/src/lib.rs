use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)] // bind function to wasm. this is where the program starts (runs automatically)
pub fn main() -> Result<(), JsValue> { // no parameters, return javascript value
    wasm_logger::init(wasm_logger::Config::default()); // allow wasm to connect to browser console
    log::info!("hello world");
    Ok(())
}

#[wasm_bindgen] // bind function to wasm. function does not run automatically (must be called)
pub fn add(a: u32, b: u32) -> u32 { // two int params, one int return value
    log::info!("{} + {} =", a, b);
    a + b // return
}

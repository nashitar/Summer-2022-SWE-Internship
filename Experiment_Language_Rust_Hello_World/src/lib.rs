use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)] // links method to WASM and lets WASM know that this is where to begin 
pub fn main() -> Result<(), JsValue> { // no parameters, returning javascript value
    wasm_logger::init(wasm_logger::Config::default()); // initialize console log
    log::info!("hello world"); // send to browser console
    Ok(()) // function is complete
}

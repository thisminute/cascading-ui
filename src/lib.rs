#[macro_use]
mod cwf;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {

	cwf!{
		span {
			text: hello;
		}
		div {
			text: world;
		}
	}

	Ok(())
}

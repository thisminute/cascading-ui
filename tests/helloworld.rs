// #![cfg(target_arch = "wasm32")]
use cwf::cwf;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwf! {
	text {
		content: "hello world";
	}
}

#[wasm_bindgen_test]
fn pass() {
	run().unwrap();
}

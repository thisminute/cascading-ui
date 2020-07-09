extern crate cwf;
use cwf::cwf;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwf! {
	title: "Test Website Title";
}

#[wasm_bindgen_test]
fn pass() {
	run().unwrap();
}

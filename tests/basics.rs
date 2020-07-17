extern crate cwf;
use cwf::{cwf_dom, cwf_lib};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwf_lib!();

#[wasm_bindgen_test]
fn empty() {
	cwf_dom! {}
}

#[wasm_bindgen_test]
fn instance() {
	cwf_dom! {
		thingy {}
	}
}

#[wasm_bindgen_test]
fn class() {
	cwf_dom! {
		.thingy {}
		thingy {}
	}
}

#[wasm_bindgen_test]
fn class2() {
	cwf_dom! {
		thingy {}
		.thingy {}
	}
}

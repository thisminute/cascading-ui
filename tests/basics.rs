extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_dom, cwl_lib};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_lib!();

#[wasm_bindgen_test]
fn empty() {
	cwl_dom! {
		title: "test";
	}
}

#[wasm_bindgen_test]
fn instance() {
	cwl_dom! {
		title: "test";
		thingy {}
	}
}

#[wasm_bindgen_test]
fn class() {
	cwl_dom! {
		title: "test";
		.thingy {}
		thingy {}
	}
}

#[wasm_bindgen_test]
fn class2() {
	cwl_dom! {
		title: "test";
		thingy {}
		.thingy {}
	}
}

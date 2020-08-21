extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_document, cwl_header};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn empty() {
	cwl_document! {
		title: "test";
	}
}

#[wasm_bindgen_test]
fn instance() {
	cwl_document! {
		title: "test";
		thingy {}
	}
}

#[wasm_bindgen_test]
fn class() {
	cwl_document! {
		title: "test";
		.thingy {}
		thingy {}
	}
}

#[wasm_bindgen_test]
fn class2() {
	cwl_document! {
		title: "test";
		thingy {}
		.thingy {}
	}
}

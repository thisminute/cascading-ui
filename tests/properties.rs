extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_document, cwl_header};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn title() {
	cwl_document! {
		title: "test";
	}
}

#[wasm_bindgen_test]
fn text() {
	cwl_document! {
		title: "test";
		// cannot set text of body in tests: https://github.com/rustwasm/wasm-bindgen/issues/2235
		// text: "hello";

		text {
			text: "world";
		}
	}
}

#[wasm_bindgen_test]
fn link() {
	cwl_document! {
		title: "test";
		link: "hello";
	}
}

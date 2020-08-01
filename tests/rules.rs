extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_dom, cwl_lib};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_lib!();

#[wasm_bindgen_test]
fn title() {
	cwl_dom! {
		title: "hello";
	}
}

#[wasm_bindgen_test]
fn text() {
	cwl_dom! {
		// cannot set text of body in tests: https://github.com/rustwasm/wasm-bindgen/issues/2235
		// text: "hello";

		text {
			text: "world";
		}
	}
}

#[wasm_bindgen_test]
fn link() {
	cwl_dom! {
		link: "hello";
	}
}

extern crate cwf;
use cwf::{cwf_dom, cwf_lib};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwf_lib!();

#[wasm_bindgen_test]
fn title() {
	cwf_dom! {
		title: "hello";
	}
}

#[wasm_bindgen_test]
fn text() {
	cwf_dom! {
		// cannot set text of body in tests: https://github.com/rustwasm/wasm-bindgen/issues/2235
		// text: "hello";

		text {
			text: "world";
		}
	}
}

#[wasm_bindgen_test]
fn link() {
	cwf_dom! {
		link: "hello";
	}
}

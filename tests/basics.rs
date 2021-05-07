extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_header, cwl_test_setup};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn empty() {
	cwl_test_setup! {}
	assert_eq!(root.inner_html(), "", "the root node should be empty");
}

#[wasm_bindgen_test]
fn element() {
	cwl_test_setup! {
		thingy {}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		""
	);
}

#[wasm_bindgen_test]
fn property() {
	cwl_test_setup! {
		thingy {
			text: "hello world";
		}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn class() {
	cwl_test_setup! {
		.thingy {
			text: "hello world";
		}
		thingy {}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}

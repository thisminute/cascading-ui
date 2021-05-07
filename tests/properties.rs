extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_header, cwl_test_setup};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn title() {
	cwl_test_setup! {
		title: "hello world";
	}
}

#[wasm_bindgen_test]
fn text() {
	cwl_test_setup! {
		text: "hello";
		text {
			text: "world";
		}
		?click {
			text: "!";
		}
	}
	assert_eq!(
		root.first_child()
			.expect("the root should contain a node")
			.text_content()
			.expect("the node should contain text"),
		"hello"
	);
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"world"
	);
	root.click();
	assert_eq!(
		root.first_child()
			.expect("the root should still contain a node")
			.text_content()
			.expect("the node should still contain text"),
		"!"
	);
}

#[wasm_bindgen_test]
fn link() {
	cwl_test_setup! {
		link: "hello";
	}
}

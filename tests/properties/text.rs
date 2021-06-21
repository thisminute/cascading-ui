extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::*,
};

test_header!();

#[wasm_bindgen_test]
fn base() {
	test_setup! {
		text: "hello world";
	}
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn element() {
	test_setup! {
		text {
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
fn event() {
	test_setup! {
		text: "click me";
		?click {
			text: "hello world";
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

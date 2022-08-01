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
		.thingy {
			text: "hello world";
		}
		thingy {}
	}
	assert_eq!(
		root
			.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn hoisting() {
	test_setup! {
		thingy {}
		.thingy {
			text: "hello world";
		}
	}
	assert_eq!(
		root
			.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn priority() {
	test_setup! {
		a {
			text: "hello world";
		}
		.a {
			text: "hi";
		}
	}
	assert_eq!(
		root
			.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}

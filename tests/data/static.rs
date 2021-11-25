extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::*,
};

wasm_bindgen_test_configure!(run_in_browser);
test_header!();

#[wasm_bindgen_test]
fn base() {
	test_setup! {
		$text: "hello world";
		text: $text;
	}
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn class_1() {
	test_setup! {
		a {
			text: $text;
		}
		.a {
			$text: "hello world";
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
fn class_2() {
	test_setup! {
		a {
			$text: "hello world";
		}
		.a {
			text: $text;
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

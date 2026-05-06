extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn base() {
	test_setup! {
		let $text: "hello world";
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
			let $text: "hello world";
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
fn class_2() {
	test_setup! {
		a {
			let $text: "hello world";
		}
		.a {
			text: $text;
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
fn priority_1() {
	test_setup! {
		a {
			let $text: "hello world";
			text: $text;
		}
		.a {
			let $text: "hi";
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
fn priority_2() {
	test_setup! {
		a {
			let $text: "hello world";
		}
		.a {
			let $text: "hi";
			text: $text;
		}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}

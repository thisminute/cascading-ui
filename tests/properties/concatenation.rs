extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn string_literals() {
	test_setup! {
		text: "Hello, " "World" "!";
	}
	assert_eq!(root.inner_html(), "Hello, World!");
}

#[wasm_bindgen_test]
fn with_variable() {
	test_setup! {
		let $name: "World";
		text: "Hello, " $name "!";
	}
	assert_eq!(root.inner_html(), "Hello, World!");
}

#[wasm_bindgen_test]
fn variable_only_unchanged() {
	test_setup! {
		let $greeting: "hi";
		text: $greeting;
	}
	assert_eq!(root.inner_html(), "hi");
}

#[wasm_bindgen_test]
fn in_child_element() {
	test_setup! {
		let $color: "red";
		item {
			text: "Color: " $color;
		}
	}
	let el = root.first_element_child().expect("should have child");
	assert_eq!(el.inner_html(), "Color: red");
}

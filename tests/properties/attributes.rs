extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn aria_label() {
	test_setup! {
		item {
			aria-label: "Main navigation";
			text: "Home";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("aria-label").expect("should have aria-label"),
		"Main navigation"
	);
}

#[wasm_bindgen_test]
fn data_attribute() {
	test_setup! {
		item {
			data-id: "42";
			text: "Item";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("data-id").expect("should have data-id"),
		"42"
	);
}

#[wasm_bindgen_test]
fn role_attribute() {
	test_setup! {
		nav {
			role: "navigation";
			text: "Menu";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("role").expect("should have role"),
		"navigation"
	);
}

#[wasm_bindgen_test]
fn tabindex_attribute() {
	test_setup! {
		item {
			tabindex: "0";
			text: "Focusable";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("tabindex").expect("should have tabindex"),
		"0"
	);
}

#[wasm_bindgen_test]
fn attribute_from_class() {
	test_setup! {
		.item {
			data-category: "button";
		}
		item {
			text: "Click";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("data-category").expect("should have data-category"),
		"button"
	);
}

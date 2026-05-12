extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn aria_label_on_element() {
	test_setup! {
		nav {
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
fn aria_label_from_class() {
	test_setup! {
		.item {
			aria-label: "Menu item";
		}
		item {
			text: "Click me";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("aria-label").expect("should have aria-label"),
		"Menu item"
	);
}

#[wasm_bindgen_test]
fn aria_hidden_on_element() {
	test_setup! {
		decorative {
			aria-hidden: "true";
			text: "decorative";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("aria-hidden").expect("should have aria-hidden"),
		"true"
	);
}

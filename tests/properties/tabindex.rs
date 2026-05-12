extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn tabindex_on_element() {
	test_setup! {
		item {
			text: "focusable";
			tabindex: "0";
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
fn tabindex_from_class() {
	test_setup! {
		.item {
			tabindex: "0";
		}
		item {
			text: "click me";
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
fn tabindex_negative() {
	test_setup! {
		item {
			text: "not focusable";
			tabindex: "-1";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("tabindex").expect("should have tabindex"),
		"-1"
	);
}

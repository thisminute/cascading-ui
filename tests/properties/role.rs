extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn role_on_element() {
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
fn role_from_class() {
	test_setup! {
		.item {
			role: "button";
		}
		item {
			text: "Click me";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(
		el.get_attribute("role").expect("should have role"),
		"button"
	);
}

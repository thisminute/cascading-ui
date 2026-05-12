extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn list_style_type() {
	test_setup! {
		item {
			list-style-type: "none";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	let style = window.get_computed_style(&el).unwrap().unwrap();
	assert_eq!(
		style.get_property_value("list-style-type").unwrap(),
		"none"
	);
}

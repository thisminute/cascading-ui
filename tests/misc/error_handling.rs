extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Verify that valid properties work alongside each other
#[wasm_bindgen_test]
fn multiple_valid_properties() {
	test_setup! {
		text: "hello";
		color: "red";
		display: "block";
	}
	assert_eq!(root.inner_html(), "hello");
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

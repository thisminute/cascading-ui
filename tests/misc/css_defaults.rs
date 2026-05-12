extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn default_box_sizing() {
	test_setup! {
		text: "box";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(
		style.get_property_value("box-sizing").unwrap(),
		"border-box"
	);
}

#[wasm_bindgen_test]
fn box_sizing_with_padding() {
	test_setup! {
		width: "100px";
		padding: "10px";
		text: "padded";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	// With border-box, the total width includes padding
	// So computed width should be 100px (not 120px)
	assert_eq!(style.get_property_value("width").unwrap(), "100px");
}

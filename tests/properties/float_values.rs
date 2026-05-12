extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn float_opacity() {
	test_setup! {
		opacity: 0.5;
		text: "transparent";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("opacity").unwrap(), "0.5");
}

#[wasm_bindgen_test]
fn float_line_height() {
	test_setup! {
		line-height: 1.5;
		text: "spaced";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	// Browsers normalize line-height to a computed value
	let lh = style.get_property_value("line-height").unwrap();
	assert!(lh.len() > 0, "line-height should be set");
}

#[wasm_bindgen_test]
fn float_flex_grow() {
	test_setup! {
		display: "flex";
		child {
			flex-grow: 2.5;
			text: "flex child";
		}
	}
	let child = root.first_element_child().unwrap();
	let style = window.get_computed_style(&child).unwrap().unwrap();
	assert_eq!(style.get_property_value("flex-grow").unwrap(), "2.5");
}

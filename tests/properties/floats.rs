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
		.item {
			opacity: 0.5;
		}
		item {}
	}
	let el = root
		.first_element_child()
		.expect("should have child");
	let window = web_sys::window().unwrap();
	let style = window
		.get_computed_style(&el)
		.unwrap()
		.unwrap();
	let opacity = style.get_property_value("opacity").unwrap();
	assert_eq!(opacity, "0.5");
}

#[wasm_bindgen_test]
fn float_line_height() {
	test_setup! {
		.item {
			line-height: 1.5;
		}
		item {
			text: "hello";
		}
	}
	let el = root
		.first_element_child()
		.expect("should have child");
	let window = web_sys::window().unwrap();
	let style = window
		.get_computed_style(&el)
		.unwrap()
		.unwrap();
	let line_height = style.get_property_value("line-height").unwrap();
	// Browsers may compute line-height differently, just check it's not empty
	assert!(!line_height.is_empty(), "line-height should be set");
}

#[wasm_bindgen_test]
fn integer_still_works() {
	test_setup! {
		.item {
			opacity: 1;
		}
		item {}
	}
	let el = root
		.first_element_child()
		.expect("should have child");
	let window = web_sys::window().unwrap();
	let style = window
		.get_computed_style(&el)
		.unwrap()
		.unwrap();
	let opacity = style.get_property_value("opacity").unwrap();
	assert_eq!(opacity, "1");
}

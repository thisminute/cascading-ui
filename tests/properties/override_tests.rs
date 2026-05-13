extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn later_property_overrides_earlier() {
	test_setup! {
		text: "first";
		text: "second";
	}
	assert_eq!(root.inner_html(), "second");
}

#[wasm_bindgen_test]
fn class_property_applied_to_element() {
	test_setup! {
		.styled {
			color: "blue";
		}
		styled {
			text: "colored";
		}
	}
	let child = root.first_element_child().unwrap();
	let style = window.get_computed_style(&child).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
}

#[wasm_bindgen_test]
fn multiple_css_properties() {
	test_setup! {
		color: "green";
		font-size: "20px";
		text: "styled";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 128, 0)");
	assert_eq!(style.get_property_value("font-size").unwrap(), "20px");
}

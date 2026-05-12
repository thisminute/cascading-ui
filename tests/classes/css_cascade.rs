extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// CSS properties from a class should apply via computed style
#[wasm_bindgen_test]
fn class_css_applies() {
	test_setup! {
		.styled {
			color: "red";
			background-color: "blue";
		}
		styled {
			text: "hello";
		}
	}
	let el = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&el).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
	assert_eq!(
		style.get_property_value("background-color").unwrap(),
		"rgb(0, 0, 255)"
	);
}

/// Element inline CSS properties should override class CSS
#[wasm_bindgen_test]
fn element_css_overrides_class() {
	test_setup! {
		.item {
			color: "red";
		}
		item {
			color: "blue";
			text: "override";
		}
	}
	let el = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&el).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
}

/// CSS class properties should apply to all matching elements
#[wasm_bindgen_test]
fn class_css_applies_to_multiple() {
	test_setup! {
		.item {
			color: "green";
		}
		item {
			text: "first";
		}
		item {
			text: "second";
		}
	}
	let first = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let second = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
	let style1 = window.get_computed_style(&first).unwrap().unwrap();
	let style2 = window.get_computed_style(&second).unwrap().unwrap();
	assert_eq!(style1.get_property_value("color").unwrap(), "rgb(0, 128, 0)");
	assert_eq!(style2.get_property_value("color").unwrap(), "rgb(0, 128, 0)");
}

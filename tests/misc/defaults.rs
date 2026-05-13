extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Body should have margin: 0 as a default CSS reset
#[wasm_bindgen_test]
fn body_margin_zero() {
	test_setup! {
		text: "test";
	}
	let style = window.get_computed_style(&body).unwrap().unwrap();
	assert_eq!(style.get_property_value("margin").unwrap(), "0px");
}

/// Link elements (<a> tags) should have display: block by default
#[wasm_bindgen_test]
fn link_display_block() {
	test_setup! {
		child {
			link: "https://example.com";
			text: "click";
		}
	}
	let child = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(child.tag_name(), "A");
	let style = window.get_computed_style(&child).unwrap().unwrap();
	assert_eq!(style.get_property_value("display").unwrap(), "block");
}

/// Elements should render as div tags by default
#[wasm_bindgen_test]
fn default_tag_is_div() {
	test_setup! {
		child {
			text: "hello";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.tag_name(), "DIV");
}

/// Root element should be a div by default
#[wasm_bindgen_test]
fn root_is_div() {
	test_setup! {
		text: "root";
	}
	assert_eq!(root.tag_name(), "DIV");
}

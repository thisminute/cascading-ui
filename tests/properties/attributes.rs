extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// Unrecognized properties should become HTML attributes
#[wasm_bindgen_test]
fn unrecognized_becomes_attribute() {
	test_setup! {
		item {
			id: "my-item";
			text: "content";
		}
	}
	let el = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(el.get_attribute("id").unwrap(), "my-item");
}

// Multiple attributes on same element
#[wasm_bindgen_test]
fn multiple_attributes() {
	test_setup! {
		item {
			id: "test";
			role: "button";
			tabindex: "0";
		}
	}
	let el = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(el.get_attribute("id").unwrap(), "test");
	assert_eq!(el.get_attribute("role").unwrap(), "button");
	assert_eq!(el.get_attribute("tabindex").unwrap(), "0");
}

// Attributes with data- prefix
#[wasm_bindgen_test]
fn data_attributes() {
	test_setup! {
		item {
			data-value: "42";
			data-type: "number";
		}
	}
	let el = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(el.get_attribute("data-value").unwrap(), "42");
	assert_eq!(el.get_attribute("data-type").unwrap(), "number");
}

// CSS properties should still go to style, not attributes
#[wasm_bindgen_test]
fn css_still_works() {
	test_setup! {
		item {
			color: "red";
			id: "styled";
		}
	}
	let el = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	// color should be inline style, not attribute
	assert_eq!(el.style().get_property_value("color").unwrap(), "red");
	// id should be attribute
	assert_eq!(el.get_attribute("id").unwrap(), "styled");
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// @if with a truthy variable — content should be visible
#[wasm_bindgen_test]
fn conditional_true() {
	test_setup! {
		let $show: "true";
		@if $show {
			content {
				text: "visible";
			}
		}
	}
	let wrapper = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&wrapper).unwrap().unwrap();
	assert_ne!(style.get_property_value("display").unwrap(), "none");
	let content = wrapper.first_element_child().unwrap();
	assert_eq!(content.inner_html(), "visible");
}

/// @if with a falsy variable — content should be hidden
#[wasm_bindgen_test]
fn conditional_false() {
	test_setup! {
		let $show: "false";
		@if $show {
			content {
				text: "hidden";
			}
		}
	}
	let wrapper = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&wrapper).unwrap().unwrap();
	assert_eq!(style.get_property_value("display").unwrap(), "none");
}

/// @if that toggles from false to true on click
#[wasm_bindgen_test]
fn conditional_toggle() {
	test_setup! {
		let $show: "false";
		?click {
			$show: "true";
		}
		@if $show {
			content {
				text: "now visible";
			}
		}
	}
	let wrapper = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&wrapper).unwrap().unwrap();
	assert_eq!(style.get_property_value("display").unwrap(), "none");
	root.click();
	let style = window.get_computed_style(&wrapper).unwrap().unwrap();
	assert_ne!(style.get_property_value("display").unwrap(), "none");
}

/// @if with empty string — should be falsy (hidden)
#[wasm_bindgen_test]
fn conditional_empty_string_is_falsy() {
	test_setup! {
		let $val: "";
		@if $val {
			content {
				text: "should be hidden";
			}
		}
	}
	let wrapper = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&wrapper).unwrap().unwrap();
	assert_eq!(style.get_property_value("display").unwrap(), "none");
}

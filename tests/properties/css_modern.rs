extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn order_property() {
	test_setup! {
		display: "flex";
		item {
			order: 2;
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element");
	let style = window
		.get_computed_style(&element)
		.unwrap()
		.unwrap();
	assert_eq!(style.get_property_value("order").unwrap(), "2");
}

#[wasm_bindgen_test]
fn pointer_events_property() {
	test_setup! {
		item {
			pointer-events: "none";
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element");
	let style = window
		.get_computed_style(&element)
		.unwrap()
		.unwrap();
	assert_eq!(style.get_property_value("pointer-events").unwrap(), "none");
}

#[wasm_bindgen_test]
fn user_select_property() {
	test_setup! {
		item {
			user-select: "none";
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element");
	let style = window
		.get_computed_style(&element)
		.unwrap()
		.unwrap();
	assert_eq!(style.get_property_value("user-select").unwrap(), "none");
}

#[wasm_bindgen_test]
fn aspect_ratio_property() {
	test_setup! {
		item {
			aspect-ratio: "16 / 9";
			width: "160";
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element");
	let style = window
		.get_computed_style(&element)
		.unwrap()
		.unwrap();
	assert_eq!(style.get_property_value("aspect-ratio").unwrap(), "16 / 9");
}

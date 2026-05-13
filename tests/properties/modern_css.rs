extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn pointer_events_property() {
	test_setup! {
		pointer-events: "none";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("pointer-events").unwrap(), "none");
}

#[wasm_bindgen_test]
fn user_select_property() {
	test_setup! {
		user-select: "none";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("user-select").unwrap(), "none");
}

#[wasm_bindgen_test]
fn filter_property() {
	test_setup! {
		filter: "blur(5px)";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert!(
		style.get_property_value("filter").unwrap().contains("blur"),
		"filter should contain blur"
	);
}

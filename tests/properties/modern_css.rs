extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Modern CSS properties should be recognized and applied
#[wasm_bindgen_test]
fn aspect_ratio() {
	test_setup! {
		aspect-ratio: "16/9";
		width: "160px";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	let ratio = style.get_property_value("aspect-ratio").unwrap();
	assert!(
		ratio.contains("16") && ratio.contains("9"),
		"aspect-ratio should be set, got: {}",
		ratio
	);
}

/// pointer-events CSS property
#[wasm_bindgen_test]
fn pointer_events() {
	test_setup! {
		pointer-events: "none";
		text: "unclickable";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("pointer-events").unwrap(), "none");
}

/// user-select CSS property
#[wasm_bindgen_test]
fn user_select() {
	test_setup! {
		user-select: "none";
		text: "not selectable";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("user-select").unwrap(), "none");
}

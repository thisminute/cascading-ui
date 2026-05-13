extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn last_text_property_wins() {
	test_setup! {
		text: "first";
		text: "second";
	}
	assert_eq!(root.inner_html(), "second");
}

#[wasm_bindgen_test]
fn last_css_property_wins() {
	test_setup! {
		color: "red";
		color: "blue";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
}

#[wasm_bindgen_test]
fn class_property_overridden_by_element() {
	test_setup! {
		.item {
			text: "class value";
			color: "red";
		}
		item {
			text: "element value";
		}
	}
	let item = root.first_element_child().expect("should have item");
	// Element's text should override class text
	assert_eq!(item.inner_html(), "element value");
	// But class CSS should still apply (element didn't set color)
	let style = window.get_computed_style(&item).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

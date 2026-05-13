extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn image_sets_background_image_style() {
	test_setup! {
		image: "https://example.com/photo.png";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	let bg = style.get_property_value("background-image").unwrap();
	assert!(
		bg.contains("example.com/photo.png"),
		"background-image should contain the URL, got: {}",
		bg
	);
}

#[wasm_bindgen_test]
fn image_on_child_element() {
	test_setup! {
		child {
			image: "https://example.com/child.jpg";
		}
	}
	let child = root.first_element_child().expect("should have child");
	let style = window.get_computed_style(&child).unwrap().unwrap();
	let bg = style.get_property_value("background-image").unwrap();
	assert!(
		bg.contains("example.com/child.jpg"),
		"background-image should contain the URL, got: {}",
		bg
	);
}

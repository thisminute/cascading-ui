extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// image: should set background-image CSS property
#[wasm_bindgen_test]
fn image_sets_background() {
	test_setup! {
		image: "https://example.com/photo.jpg";
		width: "100px";
		height: "100px";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	let bg = style.get_property_value("background-image").unwrap();
	assert!(
		bg.contains("https://example.com/photo.jpg"),
		"background-image should contain the URL, got: {}",
		bg
	);
}

/// image: on a child element
#[wasm_bindgen_test]
fn image_on_child() {
	test_setup! {
		child {
			image: "https://example.com/photo.jpg";
			width: "50px";
			height: "50px";
		}
	}
	let child = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&child).unwrap().unwrap();
	let bg = style.get_property_value("background-image").unwrap();
	assert!(
		bg.contains("https://example.com/photo.jpg"),
		"background-image should contain the URL, got: {}",
		bg
	);
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn image_sets_background() {
	test_setup! {
		image: "test.png";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	let bg = style.get_property_value("background-image").unwrap();
	assert!(bg.contains("test.png"), "background-image should contain test.png, got: {}", bg);
}

#[wasm_bindgen_test]
fn image_on_child() {
	test_setup! {
		banner {
			image: "hero.jpg";
		}
	}
	let child = root.first_element_child().unwrap();
	let child: web_sys::HtmlElement = child.dyn_into().unwrap();
	let style = window.get_computed_style(&child).unwrap().unwrap();
	let bg = style.get_property_value("background-image").unwrap();
	assert!(bg.contains("hero.jpg"), "background-image should contain hero.jpg, got: {}", bg);
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// image: should create an <img> element with src attribute
#[wasm_bindgen_test]
fn image_creates_img() {
	test_setup! {
		image: "photo.png";
	}
	let img = root.query_selector("img").unwrap().unwrap();
	assert_eq!(img.get_attribute("src").unwrap(), "photo.png");
}

// image with text should have both
#[wasm_bindgen_test]
fn image_with_text() {
	test_setup! {
		text: "Caption";
		image: "photo.png";
	}
	// Text should be present
	assert!(root.inner_html().contains("Caption"));
	// Img element should exist with correct src
	let img = root.query_selector("img").unwrap().unwrap();
	assert_eq!(img.get_attribute("src").unwrap(), "photo.png");
}

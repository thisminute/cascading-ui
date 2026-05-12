extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn href_creates_link() {
	test_setup! {
		item {
			href: "https://example.com";
			text: "click me";
		}
	}
	let element = root
		.first_element_child()
		.expect("root should have a child element");
	assert_eq!(element.tag_name(), "A");
	assert_eq!(
		element.get_attribute("href").unwrap(),
		"https://example.com"
	);
	assert_eq!(element.inner_html(), "click me");
}

#[wasm_bindgen_test]
fn href_from_class() {
	test_setup! {
		.nav {
			href: "https://example.com";
		}
		nav {}
	}
	let element = root
		.first_element_child()
		.expect("root should have a child element");
	assert_eq!(element.tag_name(), "A");
	assert_eq!(
		element.get_attribute("href").unwrap(),
		"https://example.com"
	);
}

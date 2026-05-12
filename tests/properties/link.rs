extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// link: property should create an <a> tag with href attribute
#[wasm_bindgen_test]
fn link_creates_anchor() {
	test_setup! {
		link: "https://example.com";
		text: "click here";
	}
	assert_eq!(root.tag_name(), "A");
	assert_eq!(root.get_attribute("href").unwrap(), "https://example.com");
	assert_eq!(root.inner_html(), "click here");
}

/// link: on a child element should make only that child an <a> tag
#[wasm_bindgen_test]
fn link_on_child() {
	test_setup! {
		child {
			link: "https://example.com";
			text: "link text";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.tag_name(), "A");
	assert_eq!(child.get_attribute("href").unwrap(), "https://example.com");
	// Parent should remain a div
	assert_eq!(root.tag_name(), "DIV");
}

/// link: from a class should cascade to matching elements
#[wasm_bindgen_test]
fn link_from_class() {
	test_setup! {
		.nav {
			link: "https://example.com";
		}
		nav {
			text: "navigation";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.tag_name(), "A");
	assert_eq!(child.get_attribute("href").unwrap(), "https://example.com");
}

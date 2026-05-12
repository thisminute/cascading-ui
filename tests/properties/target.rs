extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn target_on_link() {
	test_setup! {
		item {
			link: "https://example.com";
			target: "_blank";
			text: "external link";
		}
	}
	let element = root
		.first_element_child()
		.expect("root should have a child element");
	assert_eq!(element.tag_name(), "A");
	assert_eq!(element.get_attribute("href").unwrap(), "https://example.com");
	assert_eq!(element.get_attribute("target").unwrap(), "_blank");
}

#[wasm_bindgen_test]
fn target_from_class() {
	test_setup! {
		.external {
			link: "https://example.com";
			target: "_blank";
		}
		external {}
	}
	let element = root
		.first_element_child()
		.expect("root should have a child element");
	assert_eq!(element.tag_name(), "A");
	assert_eq!(element.get_attribute("target").unwrap(), "_blank");
}

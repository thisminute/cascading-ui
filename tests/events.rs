extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_header, cwl_test_setup};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn element() {
	cwl_test_setup! {
		text: "click me";
		?click {
			a {
				text: "hello world";
			}
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(
		root.first_element_child()
			.expect("the root should now contain an element")
			.inner_html(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn class() {
	cwl_test_setup! {
		text: "click me";
		a {}
		?click {
			.a {
				text: "hello world";
			}
		}
	}
	assert_eq!(
		root.text_content().expect("the root should contain text"),
		"click me"
	);
	let element = root
		.first_element_child()
		.expect("the root should now contain an element");
	assert_eq!(element.inner_html(), "", "the element should be empty");
	root.click();
	assert_eq!(
		element
			.text_content()
			.expect("the element should now contain text"),
		"hello world"
	);
}

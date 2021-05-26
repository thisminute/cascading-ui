extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_header, cwl_test_setup};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn hoisting() {
	cwl_test_setup! {
		thingy {}
		.thingy {
			text: "hello world";
		}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn compile() {
	cwl_test_setup! {
		.a {
			b {
				.c {
					color: "red";
				}
			}
		}

		a {
			.b {
				c {
					text: "yeaaaa";
				}
			}
		}
	}
	let element = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	assert_eq!(
		window
			.get_computed_style(&element)
			.unwrap()
			.unwrap()
			.get_property_value("color")
			.unwrap(),
		"rgb(255, 0, 0)"
	);
}

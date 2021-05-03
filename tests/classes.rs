extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_document, cwl_header};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn hoisting() {
	cwl_document! {
		title: "class hoisting";
		thingy {}
		.thingy {
			text: "hello world";
		}
	}
	assert_eq!(
		body.first_child()
			.unwrap()
			.first_child()
			.unwrap()
			.dyn_into::<HtmlElement>()
			.unwrap()
			.inner_text(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn compile() {
	// cwl_document! {
	// 	title: "complex class/element interactions";

	// 	.a {
	// 		.c {
	// 			color: "red";
	// 		}
	// 		b {}
	// 		b {
	// 			background_color: "brown";
	// 		}
	// 	}

	// 	a {
	// 		.b {
	// 			text: "hello";
	// 			background_color: "green";
	// 			c {
	// 				text: "yeaaaa";
	// 			}
	// 		}
	// 	}
	// }
	// let window = web_sys::window().expect("getting window");
	// let element = body
	// 	.first_child()
	// 	.unwrap()
	// 	.first_child()
	// 	.unwrap()
	// 	.first_child()
	// 	.unwrap()
	// 	.last_child()
	// 	.unwrap()
	// 	.dyn_into::<HtmlElement>()
	// 	.unwrap();
	// assert_eq!(element.inner_html(), "yeaaaa");
	// assert_eq!(
	// 	window
	// 		.get_computed_style(&element)
	// 		.unwrap()
	// 		.unwrap()
	// 		.get_property_value("color")
	// 		.unwrap(),
	// 	"rgb(255, 0, 0)"
	// );
}

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

// #[wasm_bindgen_test]
// fn compile() {
// 	cwl_test_setup! {
// 		.a {
// 			.c {
// 				color: "red";
// 			}
// 			b {}
// 			b {
// 				background_color: "brown";
// 			}
// 		}

// 		a {
// 			.b {
// 				text: "hello";
// 				background_color: "green";
// 				c {
// 					text: "yeaaaa";
// 				}
// 			}
// 		}
// 	}
// 	// let element = body
// 	// 	.first_child()
// 	// 	.unwrap()
// 	// 	.first_child()
// 	// 	.unwrap()
// 	// 	.first_child()
// 	// 	.unwrap()
// 	// 	.last_child()
// 	// 	.unwrap()
// 	// 	.dyn_into::<HtmlElement>()
// 	// 	.unwrap();
// 	// assert_eq!(element.inner_html(), "yeaaaa");
// 	// assert_eq!(
// 	// 	window
// 	// 		.get_computed_style(&element)
// 	// 		.unwrap()
// 	// 		.unwrap()
// 	// 		.get_property_value("color")
// 	// 		.unwrap(),
// 	// 	"rgb(255, 0, 0)"
// 	// );
// }

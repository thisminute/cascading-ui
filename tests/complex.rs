extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_header, cwl_test_setup};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn element() {
	cwl_test_setup! {
		text: "hi";
		.bongo {
			?click {
				text: "click the text under this";
				sdaf {
					text: "nope";
				}
			}
			color: "blue";
			text: "yeaaaa";
		}
		?click {
			// .sdaf {
			// 	color: "green";
			// 	text: "yep";
			// }
			bongo {}
		}
	}
}

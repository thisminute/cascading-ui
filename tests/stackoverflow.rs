extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_document, cwl_header};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn compile() {
	cwl_document! {
		title: "Stack Overflow";

		header {
			hamburger {}
			logo {}
			products {}
			search {}
			icons {
				profile {}
				inbox {}
				achievements {}
				review {}
				help {}
				// https://github.com/thisminute/cascading-wasm-language/issues/2
				// site-switcher {}
			}
		}
		content {
			mainbar {
				headline {}
				filter {}
				list {}
			}
			sidebar {
				stuff {}
			}
		}
	}
}

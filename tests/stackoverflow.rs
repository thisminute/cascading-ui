extern crate cwl;
use cwl::{cwl_dom, cwl_lib};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_lib!();

#[wasm_bindgen_test]
fn compile() {
	cwl_dom! {
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

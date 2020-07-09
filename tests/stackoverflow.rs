extern crate cwf;
use cwf::cwf;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwf! {
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

#[wasm_bindgen_test]
fn pass() {
	run().unwrap();
}

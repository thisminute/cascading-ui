extern crate cascading_wasm_language;
extern crate wasm_bindgen_test;
use self::{
	cascading_wasm_language::{cwl_header, cwl_test_setup},
	wasm_bindgen_test::*,
};

wasm_bindgen_test_configure!(run_in_browser);
cwl_header!();

#[wasm_bindgen_test]
fn empty() {
	cwl_test_setup! {}
	assert_eq!(root.inner_html(), "");
}

#[wasm_bindgen_test]
fn element() {
	cwl_test_setup! {
		thingy {}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		""
	);
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn empty() {
	test_setup! {}
	assert_eq!(root.inner_html(), "");
}

#[wasm_bindgen_test]
fn element() {
	test_setup! {
		thingy {}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		""
	);
}

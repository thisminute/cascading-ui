extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::*,
};

wasm_bindgen_test_configure!(run_in_browser);
test_header!();

#[wasm_bindgen_test]
fn base() {
	test_setup! {
		$text: "hello world";
		text: $text;
	}
	assert_eq!(root.inner_html(), "hello world");
}

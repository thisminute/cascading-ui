extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// CSS properties containing Rust keywords should parse correctly
#[wasm_bindgen_test]
fn align_self_property() {
	test_setup! {
		display: "flex";
		align-self: "center";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("align-self").unwrap(), "center");
}

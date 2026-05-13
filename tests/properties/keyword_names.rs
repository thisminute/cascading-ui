extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Property names that are Rust keywords should parse correctly.
/// CSS has properties like "align-self" where "self" is a keyword,
/// and "float" which is also a keyword.
#[wasm_bindgen_test]
fn keyword_as_property_name() {
	test_setup! {
		float: "left";
		text: "floating";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("float").unwrap(), "left");
}

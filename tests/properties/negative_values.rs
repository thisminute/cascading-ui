extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn negative_z_index() {
	test_setup! {
		z-index: -1;
		position: "relative";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("z-index").unwrap(), "-1");
}

#[wasm_bindgen_test]
fn negative_margin() {
	test_setup! {
		margin-top: "-10px";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("margin-top").unwrap(), "-10px");
}

#[wasm_bindgen_test]
fn negative_number_in_variable() {
	test_setup! {
		let $offset: -5;
		z-index: $offset;
		position: "relative";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("z-index").unwrap(), "-5");
}

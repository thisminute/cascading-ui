extern crate cascading_wasm_language;
extern crate wasm_bindgen_test;
use self::{
	cascading_wasm_language::{cwl_header, cwl_test_setup},
	wasm_bindgen_test::*,
};

cwl_header!();

#[wasm_bindgen_test]
fn base() {
	cwl_test_setup! {
		title: "hello world";
	}
}

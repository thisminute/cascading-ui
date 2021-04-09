extern crate cascading_wasm_language;
use cascading_wasm_language::{cwl_document, cwl_header};

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwl_header!();

#[wasm_bindgen_test]
fn empty() {
	cwl_document! {
		title: "just a title";
	}
	assert_eq!(
		body.first_child()
			.unwrap()
			.dyn_into::<HtmlElement>()
			.unwrap()
			.inner_html(),
		""
	);
}

#[wasm_bindgen_test]
fn element() {
	cwl_document! {
		title: "one empty element";
		thingy {}
	}
	assert_eq!(
		body.first_child()
			.unwrap()
			.first_child()
			.unwrap()
			.dyn_into::<HtmlElement>()
			.unwrap()
			.inner_html(),
		""
	);
}

#[wasm_bindgen_test]
fn property() {
	cwl_document! {
		title: "an element with one property";
		thingy {
			text: "hello world";
		}
	}
	assert_eq!(
		body.first_child()
			.unwrap()
			.first_child()
			.unwrap()
			.dyn_into::<HtmlElement>()
			.unwrap()
			.inner_html(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn class() {
	cwl_document! {
		title: "a class with one property applied to one element";
		.thingy {
			text: "hello world";
		}
		thingy {}
	}
	assert_eq!(
		body.first_child()
			.unwrap()
			.first_child()
			.unwrap()
			.dyn_into::<HtmlElement>()
			.unwrap()
			.inner_html(),
		"hello world"
	);
}

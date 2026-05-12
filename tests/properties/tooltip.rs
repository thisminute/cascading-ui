extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn static_tooltip() {
	test_setup! {
		item {
			tooltip: "hover text";
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element")
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.title(), "hover text");
}

#[wasm_bindgen_test]
fn tooltip_on_root() {
	test_setup! {
		tooltip: "root tooltip";
	}
	assert_eq!(root.title(), "root tooltip");
}

#[wasm_bindgen_test]
fn tooltip_in_listener() {
	test_setup! {
		text: "hover me";
		?click {
			tooltip: "now I have a tooltip";
		}
	}
	assert_eq!(root.title(), "");
	root.click();
	assert_eq!(root.title(), "now I have a tooltip");
}

#[wasm_bindgen_test]
fn tooltip_from_class() {
	test_setup! {
		.item {
			tooltip: "class tooltip";
		}
		item {}
	}
	let element = root
		.first_element_child()
		.expect("should have child element")
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.title(), "class tooltip");
}

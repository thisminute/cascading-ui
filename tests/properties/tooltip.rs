extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn tooltip_on_element() {
	test_setup! {
		text: "hover me";
		tooltip: "This is a tooltip";
	}
	assert_eq!(root.title(), "This is a tooltip");
}

#[wasm_bindgen_test]
fn tooltip_on_child() {
	test_setup! {
		item {
			text: "child";
			tooltip: "Child tooltip";
		}
	}
	let child = root.first_element_child().unwrap();
	let child: web_sys::HtmlElement = child.dyn_into().unwrap();
	assert_eq!(child.title(), "Child tooltip");
}

#[wasm_bindgen_test]
fn tooltip_from_class() {
	test_setup! {
		.tip {
			tooltip: "Class tooltip";
		}
		tip {
			text: "element";
		}
	}
	let child = root.first_element_child().unwrap();
	let child: web_sys::HtmlElement = child.dyn_into().unwrap();
	assert_eq!(child.title(), "Class tooltip");
}

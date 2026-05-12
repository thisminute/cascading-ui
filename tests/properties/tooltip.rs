extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// tooltip: should set the HTML title attribute for hover text
#[wasm_bindgen_test]
fn tooltip_sets_title() {
	test_setup! {
		text: "hover me";
		tooltip: "This is a tooltip";
	}
	assert_eq!(root.title(), "This is a tooltip");
}

/// tooltip: on a child element
#[wasm_bindgen_test]
fn tooltip_on_child() {
	test_setup! {
		child {
			text: "hover me";
			tooltip: "Child tooltip";
		}
	}
	let child = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(child.title(), "Child tooltip");
}

/// tooltip: from a class should cascade to matching elements
#[wasm_bindgen_test]
fn tooltip_from_class() {
	test_setup! {
		.tip {
			tooltip: "Class tooltip";
		}
		tip {
			text: "hover me";
		}
	}
	let child = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(child.title(), "Class tooltip");
}

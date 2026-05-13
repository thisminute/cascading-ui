extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn tooltip_sets_title_attribute() {
	test_setup! {
		text: "hover me";
		tooltip: "This is a tooltip";
	}
	assert_eq!(root.get_attribute("title").unwrap(), "This is a tooltip");
}

#[wasm_bindgen_test]
fn tooltip_on_child_element() {
	test_setup! {
		child {
			text: "hover me";
			tooltip: "Child tooltip";
		}
	}
	let child = root.first_element_child().expect("should have child");
	assert_eq!(child.get_attribute("title").unwrap(), "Child tooltip");
}

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
	let element = root.first_element_child().expect("should have element");
	assert_eq!(element.get_attribute("title").unwrap(), "Class tooltip");
}

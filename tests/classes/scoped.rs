extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Class defined at root cascades to matching element
#[wasm_bindgen_test]
fn class_cascades_to_named_element() {
	test_setup! {
		.item {
			text: "from class";
			color: "blue";
		}
		item {}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.inner_html(), "from class");
}

/// Element-level properties take priority over class properties
#[wasm_bindgen_test]
fn element_overrides_class() {
	test_setup! {
		.item {
			text: "from class";
		}
		item {
			text: "from element";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.inner_html(), "from element");
}

/// Two classes with same name both cascade
#[wasm_bindgen_test]
fn multiple_class_definitions() {
	test_setup! {
		.item {
			color: "red";
		}
		.item {
			text: "styled";
		}
		item {}
	}
	let child = root.first_element_child().unwrap();
	// Both class definitions should cascade their properties
	assert_eq!(child.inner_html(), "styled");
}

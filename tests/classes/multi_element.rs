extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn class_applies_to_multiple_elements() {
	test_setup! {
		.styled {
			color: "red";
			text: "item";
		}
		styled {}
		styled {}
		styled {}
	}
	let children = root.children();
	assert_eq!(children.length(), 3);
	for i in 0..3 {
		let child = children.item(i).unwrap();
		assert_eq!(child.inner_html(), "item");
	}
}

#[wasm_bindgen_test]
fn different_classes_on_different_elements() {
	test_setup! {
		.a { text: "alpha"; }
		.b { text: "beta"; }
		a { }
		b { }
	}
	let children = root.children();
	assert_eq!(children.length(), 2);
	assert_eq!(children.item(0).unwrap().inner_html(), "alpha");
	assert_eq!(children.item(1).unwrap().inner_html(), "beta");
}

#[wasm_bindgen_test]
fn class_with_nested_element() {
	test_setup! {
		.container {
			text: "parent";
			inner {
				text: "child";
			}
		}
		container {}
	}
	let container = root.first_element_child().unwrap();
	assert!(container.inner_html().contains("parent"));
	assert!(container.inner_html().contains("child"));
}

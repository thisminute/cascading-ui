extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn class_inside_element_cascades_to_child() {
	test_setup! {
		parent {
			.child_class {
				text: "styled";
			}
			child_class {}
		}
	}
	let parent = root.first_element_child().expect("should have parent");
	let child = parent.first_element_child().expect("should have child");
	assert_eq!(child.inner_html(), "styled");
}

#[wasm_bindgen_test]
fn class_inside_element_with_css() {
	test_setup! {
		container {
			.item {
				color: "blue";
			}
			item {
				text: "hello";
			}
		}
	}
	let container = root.first_element_child().expect("should have container");
	let item = container.first_element_child().expect("should have item");
	let style = window.get_computed_style(&item).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
	assert_eq!(item.inner_html(), "hello");
}

#[wasm_bindgen_test]
fn nested_class_does_not_leak_to_sibling_scope() {
	test_setup! {
		container_a {
			.thing {
				text: "from A";
			}
			thing {}
		}
		container_b {
			thing {}
		}
	}
	let containers = root.children();
	let a_child = containers.item(0).expect("container A")
		.first_element_child().expect("should have child");
	assert_eq!(a_child.inner_html(), "from A");
	let b_child = containers.item(1).expect("container B")
		.first_element_child().expect("should have child");
	// The class .thing defined inside container_a should NOT cascade to container_b's thing
	assert_eq!(b_child.inner_html(), "");
}

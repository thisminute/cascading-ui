extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// Based on disabled classes_1 test from dynamic.rs
// Root and multiple child elements all read the same mutable variable
// with different child listeners modifying it
#[wasm_bindgen_test]
fn root_and_children_share_variable() {
	test_setup! {
		text: $text;
		let $text: "hello world";
		?click {
			$text: "1";
		}

		a {
			text: $text;
		}
		b {
			text: $text;
			?click {
				$text: "2";
			}
		}
	}
	// Initial state: root and both children show "hello world"
	assert_eq!(root.first_child().unwrap().text_content().unwrap(), "hello world");
	let children = root.children();
	let a = children.item(0).expect("element a");
	let b = children.item(1).expect("element b");
	assert_eq!(a.inner_html(), "hello world");
	assert_eq!(b.inner_html(), "hello world");
	// Click root: $text becomes "1", all should update
	root.click();
	assert_eq!(root.first_child().unwrap().text_content().unwrap(), "1");
	assert_eq!(a.inner_html(), "1");
	assert_eq!(b.inner_html(), "1");
}

// Based on disabled classes_3 test from dynamic.rs
// Tests: class defined in a listener assigns a variable, and elements
// matching that class name read the variable
// WITH let declaration (this should work)
#[wasm_bindgen_test]
fn class_in_listener_assigns_declared_variable() {
	test_setup! {
		let $text: "initial";
		text: "click me";
		?click {
			.a {
				$text: "hello world";
			}
		}
		a {
			text: $text;
		}
		a {
			text: $text;
		}
	}
	let children = root.children();
	assert_eq!(children.item(0).expect("first a").inner_html(), "initial");
	assert_eq!(children.item(1).expect("second a").inner_html(), "initial");
	root.click();
	assert_eq!(children.item(0).expect("first a after click").inner_html(), "hello world");
	assert_eq!(children.item(1).expect("second a after click").inner_html(), "hello world");
}

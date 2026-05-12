extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Verifies that a dynamically created child element has a working click listener
#[wasm_bindgen_test]
fn click_creates_child_with_listener() {
	test_setup! {
		text: "click to spawn";
		?click {
			child {
				text: "click child";
				?click {
					text: "child clicked";
				}
			}
		}
	}
	assert_eq!(root.inner_html(), "click to spawn");
	root.click();
	let child = root
		.first_element_child()
		.expect("root should now have a child")
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(child.inner_html(), "click child");
	child.click();
	assert_eq!(child.inner_html(), "child clicked");
}

/// Verifies CSS properties on dynamically created elements
#[wasm_bindgen_test]
fn click_creates_styled_child() {
	test_setup! {
		text: "click me";
		?click {
			child {
				text: "styled child";
				color: "red";
			}
		}
	}
	root.click();
	let child = root
		.first_element_child()
		.expect("root should now have a child")
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(child.inner_html(), "styled child");
	let style = window.get_computed_style(&child).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

/// Verifies variable-driven text in dynamically created elements
#[wasm_bindgen_test]
fn click_creates_child_with_variable() {
	test_setup! {
		let $msg: "hello";
		text: "click me";
		?click {
			child {
				text: $msg;
			}
		}
	}
	root.click();
	let child = root
		.first_element_child()
		.expect("root should now have a child");
	assert_eq!(child.inner_html(), "hello");
}

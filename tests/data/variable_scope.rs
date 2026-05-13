extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Variable defined at root should be accessible by child elements
#[wasm_bindgen_test]
fn parent_variable_accessible_by_child() {
	test_setup! {
		let $greeting: "hello";
		child {
			text: $greeting;
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.inner_html(), "hello");
}

/// Variable defined at root should be accessible by deeply nested elements
#[wasm_bindgen_test]
fn deeply_nested_variable_access() {
	test_setup! {
		let $msg: "deep";
		outer {
			inner {
				text: $msg;
			}
		}
	}
	let outer = root.first_element_child().unwrap();
	let inner = outer.first_element_child().unwrap();
	assert_eq!(inner.inner_html(), "deep");
}

/// Mutable variable update should propagate through nested elements
#[wasm_bindgen_test]
fn mutable_variable_updates_nested_child() {
	test_setup! {
		let $status: "waiting";
		display {
			text: $status;
		}
		button {
			text: "click";
			?click {
				$status: "clicked";
			}
		}
	}
	let display = root.children().item(0).unwrap();
	let button = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(display.inner_html(), "waiting");
	button.click();
	let display = root.children().item(0).unwrap();
	assert_eq!(display.inner_html(), "clicked");
}

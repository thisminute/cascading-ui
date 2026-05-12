extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Two independent variables, each updated by different elements
#[wasm_bindgen_test]
fn two_variables_independent() {
	test_setup! {
		let $a: "initial-a";
		let $b: "initial-b";
		first {
			text: $a;
			?click {
				$a: "updated-a";
			}
		}
		second {
			text: $b;
			?click {
				$b: "updated-b";
			}
		}
	}
	let first = root.children().item(0).unwrap();
	let second = root.children().item(1).unwrap();
	assert_eq!(first.inner_html(), "initial-a");
	assert_eq!(second.inner_html(), "initial-b");

	first.dyn_into::<HtmlElement>().unwrap().click();
	let first = root.children().item(0).unwrap();
	let second = root.children().item(1).unwrap();
	assert_eq!(first.inner_html(), "updated-a");
	assert_eq!(second.inner_html(), "initial-b");

	second.dyn_into::<HtmlElement>().unwrap().click();
	let second = root.children().item(1).unwrap();
	assert_eq!(second.inner_html(), "updated-b");
}

/// One variable read by multiple elements, updated by one
#[wasm_bindgen_test]
fn shared_variable_multiple_readers() {
	test_setup! {
		let $msg: "hello";
		first {
			text: $msg;
		}
		second {
			text: $msg;
		}
		button {
			text: "click";
			?click {
				$msg: "world";
			}
		}
	}
	let first = root.children().item(0).unwrap();
	let second = root.children().item(1).unwrap();
	assert_eq!(first.inner_html(), "hello");
	assert_eq!(second.inner_html(), "hello");

	let button = root.children().item(2).unwrap();
	button.dyn_into::<HtmlElement>().unwrap().click();

	let first = root.children().item(0).unwrap();
	let second = root.children().item(1).unwrap();
	assert_eq!(first.inner_html(), "world");
	assert_eq!(second.inner_html(), "world");
}

/// Variable used for CSS property, updated on click
#[wasm_bindgen_test]
fn variable_css_toggle() {
	test_setup! {
		let $bg: "red";
		background-color: $bg;
		text: "click to toggle";
		?click {
			$bg: "blue";
		}
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(
		style.get_property_value("background-color").unwrap(),
		"rgb(255, 0, 0)"
	);
	root.click();
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(
		style.get_property_value("background-color").unwrap(),
		"rgb(0, 0, 255)"
	);
}

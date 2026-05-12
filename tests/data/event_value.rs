extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn event_value_basic() {
	test_setup! {
		let $text: "";
		input_field {
			tag: "input";
			?click {
				$text: $value;
			}
		}
		output {
			text: $text;
		}
	}
	let input_el = root.children().item(0).unwrap();
	let input = input_el.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
	input.set_value("hello");
	input_el.dyn_ref::<HtmlElement>().unwrap().click();
	let output = root.children().item(1).unwrap();
	assert_eq!(output.inner_html(), "hello");
}

#[wasm_bindgen_test]
fn event_value_updates_multiple() {
	test_setup! {
		let $text: "";
		input_field {
			tag: "input";
			?click {
				$text: $value;
			}
		}
		display {
			text: $text;
		}
	}
	let input_el = root.children().item(0).unwrap();
	let input = input_el.dyn_ref::<web_sys::HtmlInputElement>().unwrap();

	input.set_value("first");
	input_el.dyn_ref::<HtmlElement>().unwrap().click();
	let display = root.children().item(1).unwrap();
	assert_eq!(display.inner_html(), "first");

	input.set_value("second");
	input_el.dyn_ref::<HtmlElement>().unwrap().click();
	let display = root.children().item(1).unwrap();
	assert_eq!(display.inner_html(), "second");
}

#[wasm_bindgen_test]
fn event_value_empty_string() {
	test_setup! {
		let $text: "initial";
		input_field {
			tag: "input";
			?click {
				$text: $value;
			}
		}
		display {
			text: $text;
		}
	}
	let input_el = root.children().item(0).unwrap();
	// input.value() is "" by default for a fresh input element
	input_el.dyn_ref::<HtmlElement>().unwrap().click();
	let display = root.children().item(1).unwrap();
	assert_eq!(display.inner_html(), "");
}

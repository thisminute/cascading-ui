extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn from_listener() {
	test_setup! {
		let $text: "click me";
		text: $text;
		?click {
			$text: "hello world";
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn into_listener() {
	test_setup! {
		let $text: "hello world";
		text: "click me";
		?click {
			text: $text;
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn between_listeners() {
	test_setup! {
		let $text: "hello world";
		text: "click me";
		?click {
			text: $text;
		}
		a {
			?click {
				$text: "hello world";
			}
		}
	}
	assert_eq!(root.inner_html(), "click me<div></div>");
	root.click();
	assert_eq!(root.inner_html(), "hello world<div></div>");
}

// classes_2: Variable assignment inside a class block within a listener.
// Clicking root should assign $text via the .a class block, updating both a elements.
#[wasm_bindgen_test]
fn classes_2() {
	test_setup! {
		let $text: "initial";
		text: "click me";
		?click {
			.a {
				$text: "updated";
			}
		}
		a {
			text: $text;
		}
		a {
			text: $text;
		}
	}
	let a1 = root.children().item(0).unwrap();
	let a2 = root.children().item(1).unwrap();
	assert_eq!(a1.inner_html(), "initial");
	assert_eq!(a2.inner_html(), "initial");
	root.click();
	assert_eq!(a1.inner_html(), "updated");
	assert_eq!(a2.inner_html(), "updated");
}

// classes_3: Class block in listener with property assignment (not variable).
// Clicking root should set text on all .a elements directly.
#[wasm_bindgen_test]
fn classes_3() {
	test_setup! {
		text: "click me";
		?click {
			.a {
				text: "updated by class";
			}
		}
		a {
			text: "before";
		}
		a {
			text: "before";
		}
	}
	let a1 = root.children().item(0).unwrap();
	let a2 = root.children().item(1).unwrap();
	assert_eq!(a1.inner_html(), "before");
	assert_eq!(a2.inner_html(), "before");
	root.click();
	assert_eq!(a1.inner_html(), "updated by class");
	assert_eq!(a2.inner_html(), "updated by class");
}

#[wasm_bindgen_test]
fn base() {
	test_setup! {
		text: $text;
		let $text: "click me";
		?click {
			$text: "hello world";
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn multiple_clicks() {
	test_setup! {
		text: $text;
		let $text: "first";
		?click {
			$text: "second";
		}
	}
	assert_eq!(root.inner_html(), "first");
	root.click();
	assert_eq!(root.inner_html(), "second");
	// Clicking again should produce the same result (idempotent)
	root.click();
	assert_eq!(root.inner_html(), "second");
}

#[wasm_bindgen_test]
fn variable_with_css() {
	test_setup! {
		let $color: "red";
		color: $color;
		text: "styled";
		?click {
			$color: "blue";
		}
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
	root.click();
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
}

#[wasm_bindgen_test]
fn variable_in_child_element() {
	test_setup! {
		let $text: "initial";
		?click {
			$text: "updated";
		}
		child {
			text: $text;
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.inner_html(), "initial");
	root.click();
	let child = root.first_element_child().unwrap();
	assert_eq!(child.inner_html(), "updated");
}

#[wasm_bindgen_test]
fn listener_creates_element_text() {
	test_setup! {
		text: "before";
		?click {
			text: "after";
		}
	}
	assert_eq!(root.inner_html(), "before");
	root.click();
	assert_eq!(root.inner_html(), "after");
}

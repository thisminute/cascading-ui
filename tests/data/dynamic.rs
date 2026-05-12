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

// Enabled — tests for multiple elements referencing same mutable variable
#[wasm_bindgen_test]
fn classes_1() {
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
	// Initial state: all show "hello world"
	assert_eq!(root.child_nodes().item(0).unwrap().node_value().unwrap(), "hello world");
	let child_a = root.children().item(0).unwrap();
	assert_eq!(child_a.inner_html(), "hello world");
	let child_b = root.children().item(1).unwrap();
	assert_eq!(child_b.inner_html(), "hello world");

	// Click root: $text becomes "1", all should update
	root.click();
	assert_eq!(root.child_nodes().item(0).unwrap().node_value().unwrap(), "1");
	assert_eq!(child_a.inner_html(), "1");
	assert_eq!(child_b.inner_html(), "1");

	// Click b: $text becomes "2", all should update
	child_b.dyn_ref::<HtmlElement>().unwrap().click();
	assert_eq!(root.child_nodes().item(0).unwrap().node_value().unwrap(), "2");
	assert_eq!(child_a.inner_html(), "2");
	assert_eq!(child_b.inner_html(), "2");
}

// classes_2, classes_3 — original tests used pre-syntax patterns (nameless listeners,
// undeclared variables) that are no longer valid. Replaced with cleaner tests below.

#[wasm_bindgen_test]
fn class_variable_cascade() {
	test_setup! {
		let $text: "initial";
		.msg {
			text: $text;
		}
		msg {}
		msg {}
		?click {
			$text: "updated";
		}
	}
	let first = root.children().item(0).unwrap();
	let second = root.children().item(1).unwrap();
	assert_eq!(first.inner_html(), "initial");
	assert_eq!(second.inner_html(), "initial");
	root.click();
	assert_eq!(first.inner_html(), "updated");
	assert_eq!(second.inner_html(), "updated");
}

#[wasm_bindgen_test]
fn class_variable_cascade_with_css() {
	test_setup! {
		let $color: "red";
		.styled {
			color: $color;
		}
		styled {}
		styled {}
		?click {
			$color: "blue";
		}
	}
	let first = root.children().item(0).unwrap()
		.dyn_into::<HtmlElement>().unwrap();
	let second = root.children().item(1).unwrap()
		.dyn_into::<HtmlElement>().unwrap();
	let style1 = window.get_computed_style(&first).unwrap().unwrap();
	let style2 = window.get_computed_style(&second).unwrap().unwrap();
	assert_eq!(style1.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
	assert_eq!(style2.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
	root.click();
	let style1 = window.get_computed_style(&first).unwrap().unwrap();
	let style2 = window.get_computed_style(&second).unwrap().unwrap();
	assert_eq!(style1.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
	assert_eq!(style2.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
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

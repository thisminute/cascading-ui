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

// NOTE: classes_1 now works — see tests/data/class_variable.rs for the enabled version
// with proper assertions. classes_2 and classes_3 have separate issues:
//
// classes_2: uses `? { $text: ; }` (listener with no event name) which is invalid syntax
// classes_3: uses $text without `let` declaration — the variable is only assigned inside
//            a class-in-listener, so it doesn't exist in the ancestor scope where elements
//            try to read it. Fix: add `let $text: "";` at root scope.

// #[wasm_bindgen_test]
// fn classes_2() {
// 	test_setup! {
// 		text: $text;
// 		let $text: "hello world";
// 		button {
// 			text: "click me";
// 			?click {
// 				$text: "hello world";
// 			}
// 		}
// 		? {
// 			$text: ;
// 		}
// 		?click {
// 			.a {
// 				$text: "hello world";
// 			}
// 		}
// 		a {
// 			text: $text;
// 		}
// 		a {
// 			text: $text;
// 		}
// 	}
// }

// #[wasm_bindgen_test]
// fn classes_3() {
// 	test_setup! {
// 		text: "click me";
// 		?click {
// 			.a {
// 				$text: "hello world";
// 			}
// 		}
// 		a {
// 			text: $text;
// 		}
// 		a {
// 			text: $text;
// 		}
// 	}
// }

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

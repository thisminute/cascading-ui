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

// TODO: classes_2 has invalid syntax (bare `?` listener, empty `$text: ;`).
// TODO: classes_3 uses `$text` without a `let` declaration.

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
	// Root and two child elements all read $text
	assert_eq!(root.inner_html(), "hello world<div>hello world</div><div>hello world</div>");
	// Clicking root sets $text to "1" — all readers update
	root.click();
	assert_eq!(root.inner_html(), "1<div>1</div><div>1</div>");
	// Clicking the second child (b) sets $text to "2" — all readers update
	let b_element = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
	b_element.click();
	assert_eq!(root.inner_html(), "2<div>2</div><div>2</div>");
}

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

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
	let a_el = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let b_el = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();

	// Initial: all show "hello world"
	assert_eq!(a_el.inner_html(), "hello world");
	assert_eq!(b_el.inner_html(), "hello world");

	// Click root: $text becomes "1", all references update
	root.click();
	assert_eq!(a_el.inner_html(), "1");
	assert_eq!(b_el.inner_html(), "1");
}

#[wasm_bindgen_test]
fn classes_2() {
	test_setup! {
		let $text: "initial";
		button {
			text: "reset";
			?click {
				$text: "reset value";
			}
		}
		?click {
			.a {
				$text: "class updated";
			}
		}
		a {
			text: $text;
		}
		a {
			text: $text;
		}
	}
	let button_el = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let a1 = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
	let a2 = root.children().item(2).unwrap().dyn_into::<HtmlElement>().unwrap();

	// Initial
	assert_eq!(a1.inner_html(), "initial");
	assert_eq!(a2.inner_html(), "initial");

	// Click root: class .a sets $text for matching elements
	root.click();
	assert_eq!(a1.inner_html(), "class updated");
	assert_eq!(a2.inner_html(), "class updated");

	// Click button: direct assignment resets $text
	button_el.click();
	assert_eq!(a1.inner_html(), "reset value");
	assert_eq!(a2.inner_html(), "reset value");
}

#[wasm_bindgen_test]
fn classes_3() {
	test_setup! {
		let $text: "initial";
		text: "click me";
		?click {
			.a {
				$text: "updated via class";
			}
		}
		a {
			text: $text;
		}
		a {
			text: $text;
		}
	}
	let a1 = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let a2 = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();

	// Initial: both a elements show "initial"
	assert_eq!(a1.inner_html(), "initial");
	assert_eq!(a2.inner_html(), "initial");

	// Click root: class .a sets $text for matching elements, all update
	root.click();
	assert_eq!(a1.inner_html(), "updated via class");
	assert_eq!(a2.inner_html(), "updated via class");
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

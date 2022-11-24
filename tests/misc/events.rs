extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn property() {
	test_setup! {
		?click {
			text: "hello world";
		}
	}
	assert_eq!(root.inner_html(), "");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn element() {
	test_setup! {
		text: "click me";
		?click {
			a {
				text: "hello world";
			}
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(
		root.first_element_child()
			.expect("the root should now contain an element")
			.inner_html(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn element_from_class() {
	test_setup! {
		text: "click me";
		.a {
			text: "hello world";
		}
		?click {
			a {}
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(
		root.first_element_child()
			.expect("the root should now contain an element")
			.text_content()
			.expect("the element should now contain text"),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn class() {
	test_setup! {
		text: "click me";
		a {}
		?click {
			.a {
				text: "hello world";
			}
		}
	}
	assert_eq!(
		root.text_content().expect("the root should contain text"),
		"click me"
	);
	let element = root
		.first_element_child()
		.expect("the root should contain an element");
	assert_eq!(element.inner_html(), "", "the element should be empty");
	root.click();
	assert_eq!(
		element
			.text_content()
			.expect("the element should now contain text"),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn nesting_1() {
	test_setup! {
		text: "click me";
		?click {
			text: "now click me again";
			?click {
				text: "hello world";
			}
		}
	}
	assert_eq!(
		root.text_content().expect("the root should contain text"),
		"click me"
	);
	root.click();
	assert_eq!(
		root.text_content()
			.expect("the root text should have changed"),
		"now click me again"
	);
	root.click();
	assert_eq!(
		root.text_content()
			.expect("the root text should have changed again"),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn nesting_2() {
	test_setup! {
		text: "click me";
		?click {
			a {
				text: "click me too";
				?click {
					b {
						text: "hello world";
					}
				}
			}
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(
		root.first_element_child()
			.expect("the root should now contain an element")
			.inner_html(),
		"click me too",
		"the element should contain text"
	);
	root.click();
	assert_eq!(
		root.first_element_child()
			.expect("the root should still contain an element")
			.inner_html(),
		"click me too",
		"the element should still contain the same text"
	);
	root.first_element_child()
		.expect("the root should still contain an element")
		.dyn_into::<HtmlElement>()
		.expect("the element should be an html element")
		.click();
	assert_eq!(
		root.first_element_child()
			.expect("the root should still contain an element")
			.first_element_child()
			.expect("the element should now contain an element")
			.inner_html(),
		"hello world",
		"the innermost element should contain text"
	);
}

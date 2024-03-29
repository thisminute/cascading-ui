extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn dynamic() {
	test_setup! {
		text: "click me";
		.a {
			?click {
				text: "I've been clicked!";
				b {
					text: "hello world";
				}
			}
			color: "blue";
			text: "click me too";
		}
		?click {
			a {}
			.b {
				color: "green";
				text: "nope";
			}
		}
	}
	assert_eq!(
		root.first_child()
			.expect("the root should contain a node")
			.text_content()
			.expect("the node should contain text"),
		"click me"
	);
	root.click();
	assert_eq!(
		root.first_element_child()
			.expect("the root should now contain an element")
			.inner_html(),
		"click me too"
	);
	root.first_element_child()
		.expect("the root should now contain an element")
		.dyn_into::<HtmlElement>()
		.expect("this cast should work")
		.click();
	assert_eq!(
		root.first_element_child()
			.expect("the root should still contain an element")
			.first_element_child()
			.expect("that element should now contain an element")
			.inner_html(),
		"hello world"
	);
}

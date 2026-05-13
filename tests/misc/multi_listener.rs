extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn two_click_listeners_on_same_element() {
	test_setup! {
		text: "start";
		?click {
			text: "clicked";
		}
		child {
			text: "child start";
			?click {
				text: "child clicked";
			}
		}
	}
	assert_eq!(root.inner_html(), "start<div>child start</div>");
	root.click();
	assert_eq!(root.inner_html(), "clicked<div>child start</div>");
	let child = root
		.first_element_child()
		.expect("should have child element")
		.dyn_into::<HtmlElement>()
		.expect("should be an html element");
	child.click();
	assert_eq!(child.inner_html(), "child clicked");
}

#[wasm_bindgen_test]
fn listener_creates_multiple_children() {
	test_setup! {
		text: "click me";
		?click {
			first {
				text: "A";
			}
			second {
				text: "B";
			}
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	let children = root.children();
	assert_eq!(children.length(), 2);
	assert_eq!(
		children.item(0).expect("first child").inner_html(),
		"A"
	);
	assert_eq!(
		children.item(1).expect("second child").inner_html(),
		"B"
	);
}

#[wasm_bindgen_test]
fn sequential_listeners_both_execute() {
	test_setup! {
		let $counter: "0";
		text: $counter;
		?click {
			$counter: "1";
		}
	}
	assert_eq!(root.inner_html(), "0");
	root.click();
	assert_eq!(root.inner_html(), "1");
}

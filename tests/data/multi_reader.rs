extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn two_elements_read_same_variable() {
	test_setup! {
		let $msg: "initial";
		?click {
			$msg: "updated";
		}
		first {
			text: $msg;
		}
		second {
			text: $msg;
		}
	}
	let children = root.children();
	let first = children.item(0).expect("first");
	let second = children.item(1).expect("second");
	assert_eq!(first.inner_html(), "initial");
	assert_eq!(second.inner_html(), "initial");
	root.click();
	assert_eq!(first.inner_html(), "updated");
	assert_eq!(second.inner_html(), "updated");
}

#[wasm_bindgen_test]
fn root_and_child_read_same_variable() {
	test_setup! {
		let $msg: "start";
		text: $msg;
		?click {
			$msg: "end";
		}
		child {
			text: $msg;
		}
	}
	assert_eq!(root.first_child().unwrap().text_content().unwrap(), "start");
	let child = root.first_element_child().expect("child");
	assert_eq!(child.inner_html(), "start");
	root.click();
	// After click, both root's text and child's text should update
	assert_eq!(root.first_child().unwrap().text_content().unwrap(), "end");
	assert_eq!(child.inner_html(), "end");
}

#[wasm_bindgen_test]
fn child_click_updates_sibling_variable() {
	test_setup! {
		let $val: "A";
		reader {
			text: $val;
		}
		writer {
			text: "click me";
			?click {
				$val: "B";
			}
		}
	}
	let children = root.children();
	let reader = children.item(0).expect("reader");
	let writer = children.item(1).expect("writer")
		.dyn_into::<HtmlElement>()
		.expect("should be html element");
	assert_eq!(reader.inner_html(), "A");
	writer.click();
	assert_eq!(reader.inner_html(), "B");
}

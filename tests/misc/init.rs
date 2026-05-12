extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn init_sets_text() {
	test_setup! {
		?init {
			text: "initialized";
		}
	}
	assert_eq!(root.inner_html(), "initialized");
}

#[wasm_bindgen_test]
fn init_sets_variable() {
	test_setup! {
		let $text: "before";
		text: $text;
		?init {
			$text: "after";
		}
	}
	assert_eq!(root.inner_html(), "after");
}

#[wasm_bindgen_test]
fn init_on_child() {
	test_setup! {
		child {
			?init {
				text: "child initialized";
			}
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.inner_html(), "child initialized");
}

#[wasm_bindgen_test]
fn init_with_click() {
	test_setup! {
		let $text: "before";
		text: $text;
		?init {
			$text: "initialized";
		}
		?click {
			$text: "clicked";
		}
	}
	assert_eq!(root.inner_html(), "initialized");
	root.click();
	assert_eq!(root.inner_html(), "clicked");
}

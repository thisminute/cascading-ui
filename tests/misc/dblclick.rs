extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn dblclick_changes_text() {
	test_setup! {
		text: "before";
		?dblclick {
			text: "after";
		}
	}
	assert_eq!(root.inner_html(), "before");
	// Simulate dblclick event
	let event = web_sys::Event::new("dblclick").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "after");
}

#[wasm_bindgen_test]
fn dblclick_with_click() {
	test_setup! {
		text: "initial";
		?click {
			text: "clicked";
		}
		?dblclick {
			text: "double-clicked";
		}
	}
	assert_eq!(root.inner_html(), "initial");
	root.click();
	assert_eq!(root.inner_html(), "clicked");
	let event = web_sys::Event::new("dblclick").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "double-clicked");
}

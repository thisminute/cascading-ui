extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// dblclick event should trigger handler
#[wasm_bindgen_test]
fn dblclick_event() {
	test_setup! {
		text: "waiting";
		?dblclick {
			text: "double clicked";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	let event = Event::new("dblclick").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "double clicked");
}

/// mousedown event should trigger handler
#[wasm_bindgen_test]
fn mousedown_event() {
	test_setup! {
		text: "waiting";
		?mousedown {
			text: "mouse down";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	let event = Event::new("mousedown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "mouse down");
}

/// keydown event should trigger handler
#[wasm_bindgen_test]
fn keydown_event() {
	test_setup! {
		text: "waiting";
		?keydown {
			text: "key pressed";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	let event = Event::new("keydown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "key pressed");
}

/// input event should trigger handler
#[wasm_bindgen_test]
fn input_event() {
	test_setup! {
		text: "waiting";
		?input {
			text: "input received";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	let event = Event::new("input").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "input received");
}

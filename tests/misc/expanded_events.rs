extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn dblclick_event() {
	test_setup! {
		text: "double click me";
		?dblclick {
			text: "double clicked!";
		}
	}
	assert_eq!(root.inner_html(), "double click me");
	// Simulate dblclick via web_sys
	let event = web_sys::Event::new("dblclick").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "double clicked!");
}

#[wasm_bindgen_test]
fn mousedown_event() {
	test_setup! {
		text: "press me";
		?mousedown {
			text: "pressed!";
		}
	}
	assert_eq!(root.inner_html(), "press me");
	let event = web_sys::Event::new("mousedown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "pressed!");
}

#[wasm_bindgen_test]
fn keydown_event() {
	test_setup! {
		text: "type here";
		?keydown {
			text: "key pressed!";
		}
	}
	assert_eq!(root.inner_html(), "type here");
	let event = web_sys::Event::new("keydown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "key pressed!");
}

#[wasm_bindgen_test]
fn custom_event_via_add_event_listener() {
	test_setup! {
		text: "waiting";
		?customevent {
			text: "custom triggered!";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	let event = web_sys::Event::new("customevent").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "custom triggered!");
}

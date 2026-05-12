extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn mousedown_event() {
	test_setup! {
		text: "press me";
		?mousedown {
			text: "mouse down";
		}
	}
	assert_eq!(root.inner_html(), "press me");
	let event = Event::new("mousedown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "mouse down");
}

#[wasm_bindgen_test]
fn mouseup_event() {
	test_setup! {
		text: "release me";
		?mouseup {
			text: "mouse up";
		}
	}
	assert_eq!(root.inner_html(), "release me");
	let event = Event::new("mouseup").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "mouse up");
}

#[wasm_bindgen_test]
fn mousemove_event() {
	test_setup! {
		text: "move mouse";
		?mousemove {
			text: "mouse moved";
		}
	}
	assert_eq!(root.inner_html(), "move mouse");
	let event = Event::new("mousemove").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "mouse moved");
}

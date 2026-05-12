extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn animationend_event() {
	test_setup! {
		text: "animating";
		?animationend {
			text: "animation done";
		}
	}
	assert_eq!(root.inner_html(), "animating");
	let event = Event::new("animationend").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "animation done");
}

#[wasm_bindgen_test]
fn transitionend_event() {
	test_setup! {
		text: "transitioning";
		?transitionend {
			text: "transition done";
		}
	}
	assert_eq!(root.inner_html(), "transitioning");
	let event = Event::new("transitionend").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "transition done");
}

#[wasm_bindgen_test]
fn custom_event() {
	test_setup! {
		text: "waiting";
		?myevent {
			text: "custom fired";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	let event = Event::new("myevent").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "custom fired");
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn dblclick() {
	test_setup! {
		text: "double click me";
		?dblclick {
			text: "double clicked";
		}
	}
	assert_eq!(root.inner_html(), "double click me");
	let event = web_sys::Event::new("dblclick").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "double clicked");
}

#[wasm_bindgen_test]
fn input_event() {
	test_setup! {
		text: "waiting";
		?input {
			text: "got input";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	let event = web_sys::Event::new("input").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "got input");
}

#[wasm_bindgen_test]
fn change_event() {
	test_setup! {
		text: "unchanged";
		?change {
			text: "changed";
		}
	}
	assert_eq!(root.inner_html(), "unchanged");
	let event = web_sys::Event::new("change").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "changed");
}

#[wasm_bindgen_test]
fn keydown_event() {
	test_setup! {
		text: "press a key";
		?keydown {
			text: "key pressed";
		}
	}
	assert_eq!(root.inner_html(), "press a key");
	let event = web_sys::Event::new("keydown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "key pressed");
}

#[wasm_bindgen_test]
fn keyup_event() {
	test_setup! {
		text: "hold a key";
		?keyup {
			text: "key released";
		}
	}
	assert_eq!(root.inner_html(), "hold a key");
	let event = web_sys::Event::new("keyup").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "key released");
}

#[wasm_bindgen_test]
fn scroll_event() {
	test_setup! {
		text: "scroll me";
		?scroll {
			text: "scrolled";
		}
	}
	assert_eq!(root.inner_html(), "scroll me");
	let event = web_sys::Event::new("scroll").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "scrolled");
}

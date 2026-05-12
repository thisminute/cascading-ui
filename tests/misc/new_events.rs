extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

fn dispatch(element: &HtmlElement, event_name: &str) {
	let event = Event::new(event_name).unwrap();
	element.dispatch_event(&event).unwrap();
}

#[wasm_bindgen_test]
fn dblclick() {
	test_setup! {
		text: "waiting";
		?dblclick {
			text: "double clicked";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	dispatch(&root, "dblclick");
	assert_eq!(root.inner_html(), "double clicked");
}

#[wasm_bindgen_test]
fn mousedown() {
	test_setup! {
		text: "waiting";
		?mousedown {
			text: "mouse down";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	dispatch(&root, "mousedown");
	assert_eq!(root.inner_html(), "mouse down");
}

#[wasm_bindgen_test]
fn mouseup() {
	test_setup! {
		text: "waiting";
		?mouseup {
			text: "mouse up";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	dispatch(&root, "mouseup");
	assert_eq!(root.inner_html(), "mouse up");
}

#[wasm_bindgen_test]
fn mousemove() {
	test_setup! {
		text: "waiting";
		?mousemove {
			text: "mouse moved";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	dispatch(&root, "mousemove");
	assert_eq!(root.inner_html(), "mouse moved");
}

#[wasm_bindgen_test]
fn keydown() {
	test_setup! {
		text: "waiting";
		?keydown {
			text: "key pressed";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	dispatch(&root, "keydown");
	assert_eq!(root.inner_html(), "key pressed");
}

#[wasm_bindgen_test]
fn keyup() {
	test_setup! {
		text: "waiting";
		?keyup {
			text: "key released";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	dispatch(&root, "keyup");
	assert_eq!(root.inner_html(), "key released");
}

#[wasm_bindgen_test]
fn scroll() {
	test_setup! {
		text: "waiting";
		?scroll {
			text: "scrolled";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	dispatch(&root, "scroll");
	assert_eq!(root.inner_html(), "scrolled");
}

#[wasm_bindgen_test]
fn contextmenu() {
	test_setup! {
		text: "waiting";
		?contextmenu {
			text: "right clicked";
		}
	}
	assert_eq!(root.inner_html(), "waiting");
	dispatch(&root, "contextmenu");
	assert_eq!(root.inner_html(), "right clicked");
}

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
		text: "before";
		?dblclick {
			text: "after";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("dblclick").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "after");
}

#[wasm_bindgen_test]
fn mousedown() {
	test_setup! {
		text: "before";
		?mousedown {
			text: "pressed";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("mousedown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "pressed");
}

#[wasm_bindgen_test]
fn mouseup() {
	test_setup! {
		text: "before";
		?mouseup {
			text: "released";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("mouseup").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "released");
}

#[wasm_bindgen_test]
fn input_event() {
	test_setup! {
		text: "before";
		?input {
			text: "typed";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("input").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "typed");
}

#[wasm_bindgen_test]
fn change_event() {
	test_setup! {
		text: "before";
		?change {
			text: "changed";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("change").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "changed");
}

#[wasm_bindgen_test]
fn submit_event() {
	test_setup! {
		text: "before";
		?submit {
			text: "submitted";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("submit").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "submitted");
}

#[wasm_bindgen_test]
fn keydown_event() {
	test_setup! {
		text: "before";
		?keydown {
			text: "key pressed";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("keydown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "key pressed");
}

#[wasm_bindgen_test]
fn keyup_event() {
	test_setup! {
		text: "before";
		?keyup {
			text: "key released";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("keyup").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "key released");
}

#[wasm_bindgen_test]
fn scroll_event() {
	test_setup! {
		text: "before";
		?scroll {
			text: "scrolled";
		}
	}
	assert_eq!(root.inner_html(), "before");
	let event = Event::new("scroll").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "scrolled");
}

#[wasm_bindgen_test]
fn wheel_event() {
	test_setup! {
		text: "before";
		?wheel {
			text: "wheeled";
		}
	}
	assert_eq!(root.inner_html(), "before");
	// WheelEvent requires specific constructor
	let event = Event::new("wheel").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "wheeled");
}

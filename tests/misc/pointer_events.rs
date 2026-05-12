extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn pointerdown_event() {
	test_setup! {
		text: "press me";
		?pointerdown {
			text: "pressed";
		}
	}
	assert_eq!(root.inner_html(), "press me");
	let event = Event::new("pointerdown").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "pressed");
}

#[wasm_bindgen_test]
fn pointerup_event() {
	test_setup! {
		text: "release me";
		?pointerup {
			text: "released";
		}
	}
	assert_eq!(root.inner_html(), "release me");
	let event = Event::new("pointerup").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "released");
}

#[wasm_bindgen_test]
fn focusin_event() {
	test_setup! {
		text: "focus in me";
		?focusin {
			text: "focused in";
		}
	}
	assert_eq!(root.inner_html(), "focus in me");
	let event = Event::new("focusin").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "focused in");
}

#[wasm_bindgen_test]
fn focusout_event() {
	test_setup! {
		text: "focus out";
		?focusout {
			text: "focus lost";
		}
	}
	assert_eq!(root.inner_html(), "focus out");
	let event = Event::new("focusout").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "focus lost");
}

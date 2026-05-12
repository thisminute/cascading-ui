extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn contextmenu_event() {
	test_setup! {
		text: "right click me";
		?contextmenu {
			text: "context menu opened";
		}
	}
	assert_eq!(root.inner_html(), "right click me");
	let event = Event::new("contextmenu").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "context menu opened");
}

#[wasm_bindgen_test]
fn wheel_event() {
	test_setup! {
		text: "scroll wheel";
		?wheel {
			text: "wheel scrolled";
		}
	}
	assert_eq!(root.inner_html(), "scroll wheel");
	let event = Event::new("wheel").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "wheel scrolled");
}

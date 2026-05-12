extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn keypress_event() {
	test_setup! {
		text: "waiting";
		?keypress {
			text: "pressed";
		}
	}
	assert_eq!(root.inner_html(), "waiting");

	// Dispatch a keypress event
	let init = web_sys::KeyboardEventInit::new();
	init.set_key("a");
	let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
		"keypress",
		&init,
	)
	.unwrap();
	root.dispatch_event(&event).unwrap();

	assert_eq!(root.inner_html(), "pressed");
}

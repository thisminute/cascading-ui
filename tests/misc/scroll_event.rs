extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn scroll_event() {
	test_setup! {
		text: "not scrolled";
		?scroll {
			text: "scrolled";
		}
	}
	assert_eq!(root.inner_html(), "not scrolled");
	let event = Event::new("scroll").unwrap();
	root.dispatch_event(&event).unwrap();
	assert_eq!(root.inner_html(), "scrolled");
}

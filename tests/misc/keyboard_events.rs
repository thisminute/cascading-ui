extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn keydown_event() {
	test_setup! {
		item {
			text: "waiting";
			?keydown {
				text: "key pressed";
			}
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.inner_html(), "waiting");
	let event = Event::new("keydown").unwrap();
	element.dispatch_event(&event).unwrap();
	assert_eq!(element.inner_html(), "key pressed");
}

#[wasm_bindgen_test]
fn keyup_event() {
	test_setup! {
		item {
			text: "waiting";
			?keyup {
				text: "key released";
			}
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.inner_html(), "waiting");
	let event = Event::new("keyup").unwrap();
	element.dispatch_event(&event).unwrap();
	assert_eq!(element.inner_html(), "key released");
}

#[wasm_bindgen_test]
fn input_event() {
	test_setup! {
		item {
			text: "waiting";
			?input {
				text: "input received";
			}
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.inner_html(), "waiting");
	let event = Event::new("input").unwrap();
	element.dispatch_event(&event).unwrap();
	assert_eq!(element.inner_html(), "input received");
}

#[wasm_bindgen_test]
fn change_event() {
	test_setup! {
		item {
			text: "waiting";
			?change {
				text: "value changed";
			}
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.inner_html(), "waiting");
	let event = Event::new("change").unwrap();
	element.dispatch_event(&event).unwrap();
	assert_eq!(element.inner_html(), "value changed");
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn dblclick_event() {
	test_setup! {
		item {
			text: "initial";
			?dblclick {
				text: "double clicked";
			}
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.inner_html(), "initial");
	let event = Event::new("dblclick").unwrap();
	element.dispatch_event(&event).unwrap();
	assert_eq!(element.inner_html(), "double clicked");
}

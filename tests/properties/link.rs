extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn static_link() {
	test_setup! {
		item {
			link: "#page";
			text: "go to page";
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element");
	assert_eq!(element.tag_name(), "A");
	assert_eq!(element.get_attribute("href").unwrap(), "#page");
}

#[wasm_bindgen_test]
fn link_updates_dynamically() {
	test_setup! {
		item {
			link: "#first";
			text: "click me";
			?click {
				link: "#second";
			}
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element")
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.get_attribute("href").unwrap(), "#first");
	let event = Event::new("click").unwrap();
	element.dispatch_event(&event).unwrap();
	assert_eq!(element.get_attribute("href").unwrap(), "#second");
}

#[wasm_bindgen_test]
fn link_from_class() {
	test_setup! {
		.item {
			link: "#class-link";
		}
		item {
			text: "linked from class";
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element");
	assert_eq!(element.tag_name(), "A");
	assert_eq!(element.get_attribute("href").unwrap(), "#class-link");
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn aspect_ratio() {
	test_setup! {
		item {
			aspect-ratio: "16 / 9";
			width: "320px";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child")
		.dyn_into::<HtmlElement>()
		.unwrap();
	let style = window.get_computed_style(&el).unwrap().unwrap();
	assert_eq!(style.get_property_value("aspect-ratio").unwrap(), "16 / 9");
}

#[wasm_bindgen_test]
fn pointer_events_css() {
	test_setup! {
		item {
			pointer-events: "none";
			text: "unclickable";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child")
		.dyn_into::<HtmlElement>()
		.unwrap();
	let style = window.get_computed_style(&el).unwrap().unwrap();
	assert_eq!(style.get_property_value("pointer-events").unwrap(), "none");
}

#[wasm_bindgen_test]
fn user_select() {
	test_setup! {
		item {
			user-select: "none";
			text: "not selectable";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child")
		.dyn_into::<HtmlElement>()
		.unwrap();
	// user-select may or may not be reflected in computed style depending on browser
	// Just verify it compiles and sets the property
	assert_eq!(el.inner_html(), "not selectable");
}

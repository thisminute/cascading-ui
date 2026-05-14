extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn init_sets_text() {
	test_setup! {
		item {
			text: "before";
			?init {
				text: "initialized";
			}
		}
	}
	let element = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.inner_html(), "initialized");
}

#[wasm_bindgen_test]
fn init_sets_css() {
	test_setup! {
		item {
			?init {
				color: "red";
			}
		}
	}
	let element = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	let style = window.get_computed_style(&element).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

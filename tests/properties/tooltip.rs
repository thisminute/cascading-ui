extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn tooltip_on_element() {
	test_setup! {
		item {
			tooltip: "helper text";
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.title(), "helper text");
}

#[wasm_bindgen_test]
fn tooltip_from_class() {
	test_setup! {
		.tip {
			tooltip: "class tooltip";
		}
		tip {}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.title(), "class tooltip");
}

#[wasm_bindgen_test]
fn tooltip_on_click() {
	test_setup! {
		item {
			?click {
				tooltip: "clicked!";
			}
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.title(), "");
	element.click();
	assert_eq!(element.title(), "clicked!");
}

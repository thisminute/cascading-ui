extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn image_on_element() {
	test_setup! {
		item {
			image: "photo.jpg";
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	let bg = element.style().get_property_value("background-image").unwrap();
	assert!(
		bg.contains("photo.jpg"),
		"background-image should contain photo.jpg but was: {}",
		bg
	);
}

#[wasm_bindgen_test]
fn image_from_class() {
	test_setup! {
		.banner {
			image: "hero.png";
		}
		banner {}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	let bg = element.style().get_property_value("background-image").unwrap();
	assert!(
		bg.contains("hero.png"),
		"background-image should contain hero.png but was: {}",
		bg
	);
}

#[wasm_bindgen_test]
fn image_on_click() {
	test_setup! {
		item {
			?click {
				image: "clicked.jpg";
			}
		}
	}
	let element: HtmlElement = root
		.first_element_child()
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert_eq!(element.style().get_property_value("background-image").unwrap(), "");
	element.click();
	let bg = element.style().get_property_value("background-image").unwrap();
	assert!(
		bg.contains("clicked.jpg"),
		"background-image should contain clicked.jpg after click but was: {}",
		bg
	);
}

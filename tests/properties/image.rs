extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn static_image() {
	test_setup! {
		item {
			image: "photo.jpg";
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element");
	assert_eq!(element.tag_name(), "IMG");
	assert_eq!(element.get_attribute("src").unwrap(), "photo.jpg");
}

#[wasm_bindgen_test]
fn image_from_class() {
	test_setup! {
		.item {
			image: "class-photo.jpg";
		}
		item {}
	}
	let element = root
		.first_element_child()
		.expect("should have child element");
	assert_eq!(element.get_attribute("src").unwrap(), "class-photo.jpg");
}

#[wasm_bindgen_test]
fn image_in_listener() {
	test_setup! {
		item {
			?click {
				image: "clicked.jpg";
			}
		}
	}
	let element = root
		.first_element_child()
		.expect("should have child element")
		.dyn_into::<HtmlElement>()
		.unwrap();
	assert!(element.get_attribute("src").is_none());
	element.click();
	assert_eq!(element.get_attribute("src").unwrap(), "clicked.jpg");
}

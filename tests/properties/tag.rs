extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn tag_nav() {
	test_setup! {
		tag: "nav";
		text: "Navigation";
	}
	assert_eq!(root.tag_name(), "NAV");
	assert_eq!(root.inner_html(), "Navigation");
}

#[wasm_bindgen_test]
fn tag_section_child() {
	test_setup! {
		child {
			tag: "section";
			text: "Content";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.tag_name(), "SECTION");
	assert_eq!(child.inner_html(), "Content");
}

#[wasm_bindgen_test]
fn tag_from_class() {
	test_setup! {
		.heading {
			tag: "h1";
		}
		heading {
			text: "Title";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.tag_name(), "H1");
	assert_eq!(child.inner_html(), "Title");
}

#[wasm_bindgen_test]
fn tag_in_listener() {
	test_setup! {
		text: "click me";
		?click {
			tag: "span";
			text: "clicked";
			child {
				tag: "p";
				text: "paragraph";
			}
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	let child = root.first_element_child().unwrap();
	assert_eq!(child.tag_name(), "P");
	assert_eq!(child.inner_html(), "paragraph");
}

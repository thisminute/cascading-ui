extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// A class containing a child class definition should cascade correctly.
/// The outer class cascades to its named element, and the inner class
/// cascades to its named child element.
#[wasm_bindgen_test]
fn class_contains_class() {
	test_setup! {
		.card {
			text: "card content";
			.header {
				text: "card header";
			}
		}
		card {
			header {}
		}
	}
	let card = root.first_element_child().unwrap();
	// card text + header child
	assert!(card.inner_html().contains("card content"));
	let header = card.first_element_child().unwrap();
	assert_eq!(header.inner_html(), "card header");
}

/// A class with a child element definition should create those elements
/// for each matching element
#[wasm_bindgen_test]
fn class_creates_child_elements() {
	test_setup! {
		.wrapper {
			inner {
				text: "generated child";
			}
		}
		wrapper {}
	}
	let wrapper = root.first_element_child().unwrap();
	let inner = wrapper.first_element_child().unwrap();
	assert_eq!(inner.inner_html(), "generated child");
}

/// Class CSS should apply even with text cascading
#[wasm_bindgen_test]
fn class_css_with_text() {
	test_setup! {
		.styled {
			text: "styled text";
			color: "green";
		}
		styled {}
	}
	let el = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(el.inner_html(), "styled text");
	let style = window.get_computed_style(&el).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 128, 0)");
}

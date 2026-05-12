extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// tag: property should change the HTML element tag
#[wasm_bindgen_test]
fn tag_sets_element_type() {
	test_setup! {
		heading {
			tag: "h1";
			text: "Hello";
		}
	}
	let el = root.children().item(0).unwrap();
	assert_eq!(el.tag_name(), "H1");
}

// Default tag should be div
#[wasm_bindgen_test]
fn default_tag_is_div() {
	test_setup! {
		box_element {
			text: "content";
		}
	}
	let el = root.children().item(0).unwrap();
	assert_eq!(el.tag_name(), "DIV");
}

// tag: from class should cascade to element
#[wasm_bindgen_test]
fn tag_cascades_from_class() {
	test_setup! {
		.heading {
			tag: "h2";
			color: "blue";
		}
		heading {
			text: "Title";
		}
	}
	let el = root.children().item(0).unwrap();
	assert_eq!(el.tag_name(), "H2");
}

// Multiple different tags
#[wasm_bindgen_test]
fn multiple_tags() {
	test_setup! {
		title_el {
			tag: "h1";
			text: "Title";
		}
		paragraph {
			tag: "p";
			text: "Content";
		}
		footer_el {
			tag: "footer";
			text: "Footer";
		}
	}
	assert_eq!(root.children().item(0).unwrap().tag_name(), "H1");
	assert_eq!(root.children().item(1).unwrap().tag_name(), "P");
	assert_eq!(root.children().item(2).unwrap().tag_name(), "FOOTER");
}

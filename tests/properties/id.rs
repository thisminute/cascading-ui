extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn id_on_root() {
	test_setup! {
		id: "main-content";
		text: "hello";
	}
	assert_eq!(root.id(), "main-content");
}

#[wasm_bindgen_test]
fn id_on_child() {
	test_setup! {
		header {
			id: "page-header";
			text: "Header";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.id(), "page-header");
	assert_eq!(child.inner_html(), "Header");
}

#[wasm_bindgen_test]
fn id_from_class() {
	test_setup! {
		.labeled {
			id: "from-class";
		}
		labeled {
			text: "element";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.id(), "from-class");
}

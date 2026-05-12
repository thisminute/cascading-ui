extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn id_on_element() {
	test_setup! {
		child {
			id: "my-element";
			text: "Hello";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.id(), "my-element");
	assert_eq!(child.inner_html(), "Hello");
}

#[wasm_bindgen_test]
fn id_on_root() {
	test_setup! {
		id: "root-id";
		text: "Root";
	}
	assert_eq!(root.id(), "root-id");
}

#[wasm_bindgen_test]
fn id_from_class() {
	test_setup! {
		.labeled {
			id: "from-class";
		}
		labeled {
			text: "Labeled";
		}
	}
	let child = root.first_element_child().unwrap();
	assert_eq!(child.id(), "from-class");
}

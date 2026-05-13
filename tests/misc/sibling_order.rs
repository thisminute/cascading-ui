extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn siblings_maintain_order() {
	test_setup! {
		first { text: "1"; }
		second { text: "2"; }
		third { text: "3"; }
	}
	let children = root.children();
	assert_eq!(children.length(), 3);
	assert_eq!(children.item(0).unwrap().inner_html(), "1");
	assert_eq!(children.item(1).unwrap().inner_html(), "2");
	assert_eq!(children.item(2).unwrap().inner_html(), "3");
}

#[wasm_bindgen_test]
fn deep_nesting_order() {
	test_setup! {
		a {
			b {
				c {
					text: "deep";
				}
			}
		}
	}
	let a = root.first_element_child().unwrap();
	let b = a.first_element_child().unwrap();
	let c = b.first_element_child().unwrap();
	assert_eq!(c.inner_html(), "deep");
}

#[wasm_bindgen_test]
fn mixed_text_and_elements() {
	test_setup! {
		text: "parent";
		child { text: "child"; }
	}
	assert!(root.inner_html().starts_with("parent"));
	assert!(root.inner_html().contains("child"));
}

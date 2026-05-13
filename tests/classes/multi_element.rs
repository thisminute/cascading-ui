extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn class_cascades_to_multiple_matching_elements() {
	test_setup! {
		.item {
			text: "hello";
		}
		item {}
		item {}
		item {}
	}
	let children = root.children();
	assert_eq!(children.length(), 3);
	assert_eq!(children.item(0).expect("first").inner_html(), "hello");
	assert_eq!(children.item(1).expect("second").inner_html(), "hello");
	assert_eq!(children.item(2).expect("third").inner_html(), "hello");
}

#[wasm_bindgen_test]
fn class_css_cascades_to_multiple_elements() {
	test_setup! {
		.item {
			color: "red";
		}
		item {}
		item {}
	}
	let children = root.children();
	let style_0 = window.get_computed_style(&children.item(0).unwrap()).unwrap().unwrap();
	let style_1 = window.get_computed_style(&children.item(1).unwrap()).unwrap().unwrap();
	assert_eq!(style_0.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
	assert_eq!(style_1.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

#[wasm_bindgen_test]
fn different_classes_on_different_elements() {
	test_setup! {
		.alpha {
			text: "A";
		}
		.beta {
			text: "B";
		}
		alpha {}
		beta {}
	}
	let children = root.children();
	assert_eq!(children.length(), 2);
	assert_eq!(children.item(0).expect("alpha").inner_html(), "A");
	assert_eq!(children.item(1).expect("beta").inner_html(), "B");
}

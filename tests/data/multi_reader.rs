extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn variable_read_by_multiple_children() {
	test_setup! {
		let $msg: "hello";
		a { text: $msg; }
		b { text: $msg; }
		c { text: $msg; }
	}
	let children = root.children();
	assert_eq!(children.length(), 3);
	for i in 0..3 {
		assert_eq!(children.item(i).unwrap().inner_html(), "hello");
	}
}

#[wasm_bindgen_test]
fn variable_updated_propagates_to_all_readers() {
	test_setup! {
		let $msg: "before";
		?click {
			$msg: "after";
		}
		a { text: $msg; }
		b { text: $msg; }
	}
	let children = root.children();
	assert_eq!(children.item(0).unwrap().inner_html(), "before");
	assert_eq!(children.item(1).unwrap().inner_html(), "before");
	root.click();
	assert_eq!(children.item(0).unwrap().inner_html(), "after");
	assert_eq!(children.item(1).unwrap().inner_html(), "after");
}

#[wasm_bindgen_test]
fn variable_in_nested_scope() {
	test_setup! {
		let $color: "red";
		outer {
			color: $color;
			text: "outer";
			inner {
				color: $color;
				text: "inner";
			}
		}
	}
	let outer = root.first_element_child().unwrap();
	let outer_style = window.get_computed_style(&outer).unwrap().unwrap();
	assert_eq!(outer_style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
	let inner = outer.first_element_child().unwrap();
	let inner_style = window.get_computed_style(&inner).unwrap().unwrap();
	assert_eq!(inner_style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

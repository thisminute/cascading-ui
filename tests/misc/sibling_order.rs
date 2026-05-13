extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn sibling_elements_maintain_source_order() {
	test_setup! {
		first {
			text: "1";
		}
		second {
			text: "2";
		}
		third {
			text: "3";
		}
	}
	let children = root.children();
	assert_eq!(children.length(), 3);
	assert_eq!(
		children.item(0).expect("first child").inner_html(),
		"1"
	);
	assert_eq!(
		children.item(1).expect("second child").inner_html(),
		"2"
	);
	assert_eq!(
		children.item(2).expect("third child").inner_html(),
		"3"
	);
}

#[wasm_bindgen_test]
fn nested_siblings_maintain_order() {
	test_setup! {
		parent {
			alpha {
				text: "a";
			}
			beta {
				text: "b";
			}
			gamma {
				text: "c";
			}
		}
	}
	let parent = root.first_element_child().expect("should have parent");
	let children = parent.children();
	assert_eq!(children.length(), 3);
	assert_eq!(
		children.item(0).expect("first child").inner_html(),
		"a"
	);
	assert_eq!(
		children.item(1).expect("second child").inner_html(),
		"b"
	);
	assert_eq!(
		children.item(2).expect("third child").inner_html(),
		"c"
	);
}

#[wasm_bindgen_test]
fn siblings_with_mixed_content() {
	test_setup! {
		header {
			text: "Header";
		}
		content {
			left {
				text: "Left";
			}
			right {
				text: "Right";
			}
		}
		footer {
			text: "Footer";
		}
	}
	let children = root.children();
	assert_eq!(children.length(), 3);
	assert_eq!(
		children.item(0).expect("header").inner_html(),
		"Header"
	);
	let content = children.item(1).expect("content");
	let content_children = content.children();
	assert_eq!(content_children.length(), 2);
	assert_eq!(
		content_children.item(0).expect("left").inner_html(),
		"Left"
	);
	assert_eq!(
		content_children.item(1).expect("right").inner_html(),
		"Right"
	);
	assert_eq!(
		children.item(2).expect("footer").inner_html(),
		"Footer"
	);
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Apply with CSS property changes (text and color together)
#[wasm_bindgen_test]
fn apply_changes_text_and_css() {
	test_setup! {
		.highlight {
			color: "red";
			text: "highlighted";
		}
		text: "normal";
		?click {
			apply: .highlight;
		}
	}
	assert_eq!(root.inner_html(), "normal");
	root.click();
	assert_eq!(root.inner_html(), "highlighted");
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

/// Apply on a child element — not just root
#[wasm_bindgen_test]
fn apply_on_child_element() {
	test_setup! {
		.active {
			text: "active";
		}
		item {
			text: "inactive";
			?click {
				apply: .active;
			}
		}
	}
	let item = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(item.inner_html(), "inactive");
	item.click();
	assert_eq!(item.inner_html(), "active");
}

/// Two sibling elements can each use apply independently
#[wasm_bindgen_test]
fn apply_independent_siblings() {
	test_setup! {
		.on {
			text: "on";
		}
		a {
			text: "a-off";
			?click {
				apply: .on;
			}
		}
		b {
			text: "b-off";
			?click {
				apply: .on;
			}
		}
	}
	let a = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let b = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(a.inner_html(), "a-off");
	assert_eq!(b.inner_html(), "b-off");
	a.click();
	assert_eq!(a.inner_html(), "on");
	assert_eq!(b.inner_html(), "b-off");
	b.click();
	assert_eq!(a.inner_html(), "on");
	assert_eq!(b.inner_html(), "on");
}

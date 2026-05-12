extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// tooltip: should set the title attribute on the element
#[wasm_bindgen_test]
fn tooltip_sets_title() {
	test_setup! {
		text: "hover me";
		tooltip: "This is a tooltip";
	}
	assert_eq!(
		root.get_attribute("title").unwrap(),
		"This is a tooltip"
	);
}

// tooltip in a class should cascade to matching elements
#[wasm_bindgen_test]
fn tooltip_from_class() {
	test_setup! {
		.info {
			tooltip: "Info tooltip";
		}
		info {
			text: "hover me";
		}
	}
	let el = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(el.get_attribute("title").unwrap(), "Info tooltip");
}

// tooltip set dynamically via click handler
#[wasm_bindgen_test]
fn tooltip_dynamic() {
	test_setup! {
		text: "click me";
		tooltip: "before click";
		?click {
			tooltip: "after click";
		}
	}
	assert_eq!(root.get_attribute("title").unwrap(), "before click");
	root.click();
	assert_eq!(root.get_attribute("title").unwrap(), "after click");
}

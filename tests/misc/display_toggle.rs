extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// Tests the SPA-like navigation pattern: show/hide sections via mutable display variables
#[wasm_bindgen_test]
fn display_variable_toggle() {
	test_setup! {
		let $show_a: "block";
		let $show_b: "none";

		section_a {
			display: $show_a;
			text: "Page A";
		}
		section_b {
			display: $show_b;
			text: "Page B";
		}
		nav {
			text: "switch";
			?click {
				$show_a: "none";
				$show_b: "block";
			}
		}
	}
	let children = root.children();
	let section_a = children.item(0).unwrap();
	let section_b = children.item(1).unwrap();
	let nav = children.item(2).unwrap();

	// Initially: A visible, B hidden
	let style_a = window.get_computed_style(&section_a).unwrap().unwrap();
	let style_b = window.get_computed_style(&section_b).unwrap().unwrap();
	assert_eq!(style_a.get_property_value("display").unwrap(), "block");
	assert_eq!(style_b.get_property_value("display").unwrap(), "none");

	// Click nav to switch
	let nav: web_sys::HtmlElement = nav.dyn_into().unwrap();
	nav.click();

	// After click: A hidden, B visible
	let section_a = root.children().item(0).unwrap();
	let section_b = root.children().item(1).unwrap();
	let style_a = window.get_computed_style(&section_a).unwrap().unwrap();
	let style_b = window.get_computed_style(&section_b).unwrap().unwrap();
	assert_eq!(style_a.get_property_value("display").unwrap(), "none");
	assert_eq!(style_b.get_property_value("display").unwrap(), "block");
}

/// Tests the apply pattern: apply replaces element properties on click
#[wasm_bindgen_test]
fn apply_changes_display() {
	test_setup! {
		.visible {
			color: "green";
			text: "visible";
		}
		text: "click to change";
		?click {
			apply: .visible;
		}
	}
	assert_eq!(root.inner_html(), "click to change");
	root.click();
	assert_eq!(root.inner_html(), "visible");
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 128, 0)");
}

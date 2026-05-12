extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// @media with max-width larger than viewport — should apply
#[wasm_bindgen_test]
fn media_query_applies() {
	test_setup! {
		content {
			text: "hello";
			color: "blue";
			@media "(max-width: 1024px)" {
				color: "red";
			}
		}
	}
	let content = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&content).unwrap().unwrap();
	// Default viewport ~800px, so max-width: 1024px should apply
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

/// @media with min-width larger than viewport — should NOT apply
#[wasm_bindgen_test]
fn media_query_does_not_apply() {
	test_setup! {
		content {
			text: "hello";
			color: "blue";
			@media "(min-width: 1920px)" {
				color: "red";
			}
		}
	}
	let content = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&content).unwrap().unwrap();
	// Default viewport ~800px, so min-width: 1920px should NOT apply
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
}

/// @media with multiple CSS properties
#[wasm_bindgen_test]
fn media_query_multiple_props() {
	test_setup! {
		content {
			text: "hello";
			color: "blue";
			font-size: "20px";
			@media "(max-width: 1024px)" {
				color: "red";
				font-size: "14px";
			}
		}
	}
	let content = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&content).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
	assert_eq!(style.get_property_value("font-size").unwrap(), "14px");
}

/// Element with @media but no base CSS properties
#[wasm_bindgen_test]
fn media_query_no_base_css() {
	test_setup! {
		content {
			text: "hello";
			@media "(max-width: 1024px)" {
				color: "green";
			}
		}
	}
	let content = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	let style = window.get_computed_style(&content).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 128, 0)");
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn custom_property_inline() {
	test_setup! {
		--my-color: "red";
		color: "var(--my-color)";
		text: "custom";
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

#[wasm_bindgen_test]
fn custom_property_from_class() {
	test_setup! {
		.themed {
			--theme-bg: "blue";
		}
		themed {
			color: "var(--theme-bg)";
			text: "themed";
		}
	}
	let child = root.first_element_child().unwrap();
	let style = window.get_computed_style(&child).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
}

#[wasm_bindgen_test]
fn custom_property_dynamic() {
	test_setup! {
		let $color: "green";
		--text-color: $color;
		color: "var(--text-color)";
		text: "dynamic";
		?click {
			$color: "purple";
		}
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 128, 0)");
	root.click();
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(128, 0, 128)");
}

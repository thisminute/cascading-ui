extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn hover_generates_css() {
	test_setup! {
		.item {
			color: "black";
			:hover {
				color: "blue";
			}
		}
		item {}
	}
	// The element should have a CSS class
	let el = root
		.first_element_child()
		.expect("should have child");
	let class_attr = el.get_attribute("class").unwrap_or_default();
	assert!(!class_attr.is_empty(), "element should have a CSS class");

	// Check the <style> element contains the hover pseudo-class rule
	let styles = document.query_selector_all("style").unwrap();
	let last_style: HtmlElement = styles
		.item(styles.length() - 1)
		.unwrap()
		.unchecked_into();
	let css_text = last_style.inner_text();
	assert!(
		css_text.contains(":hover"),
		"CSS should contain :hover, got: {}",
		css_text
	);
	assert!(
		css_text.contains("color:blue"),
		"hover rule should have color:blue, got: {}",
		css_text
	);
}

#[wasm_bindgen_test]
fn multiple_pseudo_classes() {
	test_setup! {
		.item {
			background: "white";
			:hover {
				background: "gray";
			}
			:focus {
				outline: "2px solid blue";
			}
		}
		item {}
	}
	let styles = document.query_selector_all("style").unwrap();
	let last_style: HtmlElement = styles
		.item(styles.length() - 1)
		.unwrap()
		.unchecked_into();
	let css_text = last_style.inner_text();
	assert!(
		css_text.contains(":hover"),
		"CSS should contain :hover, got: {}",
		css_text
	);
	assert!(
		css_text.contains(":focus"),
		"CSS should contain :focus, got: {}",
		css_text
	);
}

#[wasm_bindgen_test]
fn pseudo_class_does_not_affect_dom() {
	test_setup! {
		.item {
			text: "hello";
			:hover {
				color: "red";
			}
		}
		item {}
	}
	// The element should render normally — pseudo-class doesn't add extra elements
	let el = root
		.first_element_child()
		.expect("should have child");
	assert_eq!(el.inner_html(), "hello");
	// Should have exactly one child element (no extra DOM nodes from pseudo-class)
	assert_eq!(root.children().length(), 1);
}

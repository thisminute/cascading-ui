extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// Helper to get the CSS text from the last <style> element
fn last_style_css() -> String {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let styles = document.query_selector_all("style").unwrap();
	let last = styles.item(styles.length() - 1).unwrap();
	last.dyn_into::<HtmlElement>().unwrap().inner_html()
}

// @keyframes should generate CSS animation rules
#[wasm_bindgen_test]
fn keyframes_generates_css() {
	test_setup! {
		@keyframes fadeIn {
			from {
				opacity: "0";
			}
			to {
				opacity: "1";
			}
		}

		.animated {
			animation: "fadeIn 1s ease";
		}
		animated {}
	}
	let css_text = last_style_css();
	assert!(css_text.contains("@keyframes fadeIn"), "CSS should contain @keyframes fadeIn, got: {}", css_text);
	assert!(css_text.contains("opacity:0"), "CSS should contain from properties, got: {}", css_text);
	assert!(css_text.contains("opacity:1"), "CSS should contain to properties, got: {}", css_text);
}

// Keyframes with element using the animation class
#[wasm_bindgen_test]
fn keyframes_with_animation_property() {
	test_setup! {
		@keyframes slideIn {
			from {
				transform: "translateX(-100%)";
			}
			to {
				transform: "translateX(0)";
			}
		}

		.slide {
			animation: "slideIn 0.5s ease-out";
		}
		slide {}
	}
	let css_text = last_style_css();
	assert!(css_text.contains("@keyframes slideIn"), "CSS should contain @keyframes slideIn, got: {}", css_text);
	assert!(css_text.contains("animation:slideIn 0.5s ease-out"), "CSS should contain animation property, got: {}", css_text);
}

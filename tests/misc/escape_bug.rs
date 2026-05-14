extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// ============================================================
// Attribute escaping
// ============================================================

#[wasm_bindgen_test]
fn href_with_single_quote() {
	// The compiler generates href='...' with single-quote delimiters.
	// A ' in the URL terminates the attribute early — the link breaks.
	test_setup! {
		text: "link";
		link: "https://example.com/it's-here";
	}
	let href = root.get_attribute("href").unwrap_or_default();
	web_sys::console::log_1(&format!("outer: {}", root.outer_html()).into());
	// Without escaping: href is "https://example.com/it" (truncated)
	assert_eq!(href, "https://example.com/it's-here");
}

// ============================================================
// Text content escaping
// ============================================================

#[wasm_bindgen_test]
fn text_with_html_tags() {
	// text: goes straight into innerHTML with no escaping.
	// HTML tags become real DOM elements instead of visible text.
	test_setup! {
		text: "<b>bold</b>";
	}
	web_sys::console::log_1(&format!("outer: {}", root.outer_html()).into());
	// Without escaping: creates an actual <b> element (child_element_count = 1)
	assert_eq!(root.child_element_count(), 0, "<b> in text: should not create an element");
	assert_eq!(root.inner_text(), "<b>bold</b>");
}

#[wasm_bindgen_test]
fn text_with_script_tag() {
	// A <script> tag in text: creates a real script element in the DOM.
	test_setup! {
		text: "<script>window.__xss_test=true</script>";
	}
	web_sys::console::log_1(&format!("outer: {}", root.outer_html()).into());
	// Without escaping: <script> becomes a real element
	assert_eq!(root.child_element_count(), 0, "<script> in text: should not create an element");
}

#[wasm_bindgen_test]
fn text_with_img_tag() {
	// An <img> tag in text: creates a real image element that fires a network request.
	test_setup! {
		text: "<img src=x>";
	}
	web_sys::console::log_1(&format!("outer: {}", root.outer_html()).into());
	assert_eq!(root.child_element_count(), 0, "<img> in text: should not create an element");
}

#[wasm_bindgen_test]
fn text_with_nested_elements() {
	// Multiple nested tags in text: create a whole subtree.
	test_setup! {
		text: "<div><span>nested</span></div>";
	}
	web_sys::console::log_1(&format!("outer: {}", root.outer_html()).into());
	assert_eq!(root.child_element_count(), 0, "nested tags in text: should not create elements");
	assert_eq!(root.inner_text(), "<div><span>nested</span></div>");
}

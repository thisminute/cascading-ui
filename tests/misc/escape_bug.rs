extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn href_single_quote_breaks_link() {
	// href='...' uses single-quote delimiters, so a ' in the value
	// terminates the attribute early and truncates the URL.
	test_setup! {
		text: "link";
		link: "https://example.com/it's-here";
	}
	let href = root.get_attribute("href").unwrap_or_default();
	assert_eq!(href, "https://example.com/it's-here");
}

#[wasm_bindgen_test]
fn text_html_tags_create_elements() {
	// text: goes into innerHTML unescaped, so HTML tags become real
	// DOM elements instead of visible text.
	test_setup! {
		text: "<b>bold</b>";
	}
	assert_eq!(root.child_element_count(), 0, "<b> should not become a real element");
}

#[wasm_bindgen_test]
fn text_ampersand_parsed_as_entity() {
	// A bare & in text can be parsed as an HTML entity.
	// "x &lt y" renders as "x < y" instead of the literal string.
	test_setup! {
		text: "x &lt y";
	}
	assert_eq!(root.inner_text(), "x &lt y");
}

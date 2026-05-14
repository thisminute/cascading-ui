extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn href_double_quote_breaks_link() {
	// The compiler generates href="..." with double-quote delimiters.
	// A " in the URL terminates the attribute early — the link breaks.
	test_setup! {
		text: "link";
		link: "https://example.com/a\"b";
	}
	let href = root.get_attribute("href").unwrap_or_default();
	assert_eq!(href, "https://example.com/a\"b");
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

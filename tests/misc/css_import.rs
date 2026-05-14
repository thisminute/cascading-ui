extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

/// @import URLs should be included in the generated <style> tag
#[wasm_bindgen_test]
fn import_in_style() {
	test_setup! {
		@import "https://example.com/style.css";
		item {
			text: "styled";
		}
	}

	// Find the last <style> element (our test's style)
	let styles = document.query_selector_all("style").unwrap();
	let last_style = styles
		.item(styles.length() - 1)
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	let css = last_style.inner_text();

	// The @import should be at the start of the CSS
	assert!(
		css.contains("@import url('https://example.com/style.css')"),
		"CSS should contain @import, got: {}",
		css
	);
}

/// Multiple imports should all be included
#[wasm_bindgen_test]
fn multiple_imports() {
	test_setup! {
		@import "https://example.com/a.css";
		@import "https://example.com/b.css";
		item {
			text: "styled";
		}
	}

	let styles = document.query_selector_all("style").unwrap();
	let last_style = styles
		.item(styles.length() - 1)
		.unwrap()
		.dyn_into::<HtmlElement>()
		.unwrap();
	let css = last_style.inner_text();

	assert!(
		css.contains("@import url('https://example.com/a.css')"),
		"CSS should contain first @import, got: {}",
		css
	);
	assert!(
		css.contains("@import url('https://example.com/b.css')"),
		"CSS should contain second @import, got: {}",
		css
	);
}

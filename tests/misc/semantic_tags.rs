extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn header_tag() {
	test_setup! {
		header {
			text: "Site Title";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(el.tag_name(), "HEADER");
	assert_eq!(el.inner_html(), "Site Title");
}

#[wasm_bindgen_test]
fn nav_tag() {
	test_setup! {
		nav {
			text: "Navigation";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(el.tag_name(), "NAV");
}

#[wasm_bindgen_test]
fn section_tag() {
	test_setup! {
		section {
			text: "Content";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(el.tag_name(), "SECTION");
}

#[wasm_bindgen_test]
fn footer_tag() {
	test_setup! {
		footer {
			text: "Copyright";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(el.tag_name(), "FOOTER");
}

#[wasm_bindgen_test]
fn heading_tags() {
	test_setup! {
		h1 { text: "Heading 1"; }
		h2 { text: "Heading 2"; }
		h3 { text: "Heading 3"; }
	}
	let children = root.children();
	assert_eq!(children.item(0).unwrap().tag_name(), "H1");
	assert_eq!(children.item(1).unwrap().tag_name(), "H2");
	assert_eq!(children.item(2).unwrap().tag_name(), "H3");
}

#[wasm_bindgen_test]
fn paragraph_tag() {
	test_setup! {
		p {
			text: "A paragraph of text.";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(el.tag_name(), "P");
	assert_eq!(el.inner_html(), "A paragraph of text.");
}

#[wasm_bindgen_test]
fn list_tags() {
	test_setup! {
		ul {
			li { text: "Item 1"; }
			li { text: "Item 2"; }
		}
	}
	let ul = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(ul.tag_name(), "UL");
	let items = ul.children();
	assert_eq!(items.item(0).unwrap().tag_name(), "LI");
	assert_eq!(items.item(1).unwrap().tag_name(), "LI");
}

#[wasm_bindgen_test]
fn custom_name_stays_div() {
	test_setup! {
		sidebar {
			text: "Side content";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(el.tag_name(), "DIV");
}

#[wasm_bindgen_test]
fn link_overrides_semantic() {
	test_setup! {
		nav {
			link: "https://example.com";
			text: "Link";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(el.tag_name(), "A");
}

#[wasm_bindgen_test]
fn span_tag() {
	test_setup! {
		span {
			text: "inline text";
		}
	}
	let el = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(el.tag_name(), "SPAN");
}

#[wasm_bindgen_test]
fn form_tags() {
	test_setup! {
		form {
			button {
				text: "Submit";
			}
		}
	}
	let form = root
		.first_element_child()
		.expect("root should have a child");
	assert_eq!(form.tag_name(), "FORM");
	let button = form
		.first_element_child()
		.expect("form should have a child");
	assert_eq!(button.tag_name(), "BUTTON");
}

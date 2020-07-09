use cwf::cwf;
extern crate console_error_panic_hook;

use {
	wasm_bindgen::{
		prelude::*,
		JsCast,
	},
	web_sys::{
		Document,
		HtmlElement,
	},
};

// struct Meta<'a> {
// 	window: &'a Window,
// 	document: &'a Document,
// 	head: &'a HtmlHeadElement,
// 	classes: HashMap<&'a str, Class<'a>>,
// 	elements: HashMap<&'a str, &'a HtmlElement>,
// }

// struct Class<'a> {
// 	text: &'a str,
// 	styles: Vec<&'a str>,
// }
// impl Default for Class<'_> {
// 	fn default() -> Self { Class{
// 		text: "",
// 		styles: Vec::new(),
// 	} }
// }

fn create_element(document: &Document, name: &str) -> HtmlElement {
	document
		.create_element(name)
		.expect(&format!("Failed to create `{}` element.", name)[..])
		.dyn_into::<HtmlElement>()
		.expect("Failed to construct element.")
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {			std::panic::set_hook(Box::new(console_error_panic_hook::hook));
	let window = &web_sys::window().expect("Failed to access global `window`.");
	let document = &window.document().expect("Failed to access `window.document`.");
	let head = &document.head().expect("Failed to access `window.document.head`.");
	let body = &document.body().expect("Failed to access `window.document.body`.");
	let style = &document.create_element("style").expect("Failed to create `style` element.");
	head.append_child(style).expect("Failed to append `style` element to document.");
	// let classes = HashMap::new();
	// let mut elements = HashMap::new();
	// elements.insert("body", body).unwrap();
	let current_element = body;
	cwf!();
	Ok(())
}

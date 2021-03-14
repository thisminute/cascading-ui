use {
	super::semantics::{event::EventListener, properties::CssProperties},
	std::collections::HashMap,
};

pub struct CssRule {
	pub selector: String,
	pub properties: CssProperties,
}

pub struct Element {
	pub classes: Vec<String>,
	pub style: CssProperties,
	pub children: Vec<Element>,
	pub listeners: Vec<EventListener>,
	pub text: String,
	pub link: Option<String>,
}

pub struct Page {
	pub title: String,
	pub styles: Vec<CssRule>,
	pub root: Element,
}

pub struct Dom {
	pub html_roots: HashMap<String, Page>,
}

impl Dom {
	pub fn new() -> Self {
		Self {
			html_roots: HashMap::new(),
		}
	}
}

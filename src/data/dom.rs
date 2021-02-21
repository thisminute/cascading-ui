use {
	data::semantics::{event::EventListener, rules::Rules},
	std::collections::HashMap,
};

pub struct Element {
	// pub classes: Vec<Class>,
	pub children: Vec<Element>,
	pub active: bool,
	pub listeners: Vec<EventListener>,
	pub rules: Rules,
}

pub struct Page {
	pub title: String,
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

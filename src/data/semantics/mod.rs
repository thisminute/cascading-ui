pub mod properties;

use {
	self::properties::{CssRules, Properties},
	std::collections::HashMap,
};

#[derive(Clone)]
pub struct Group {
	pub parent_id: Option<usize>,
	pub name: Option<String>,
	pub selector: Option<String>,
	pub class_names: Vec<String>,

	pub properties: Properties,
	pub elements: Vec<usize>,
	pub classes: HashMap<String, Vec<usize>>,
	pub listeners: Vec<(String, usize)>,

	pub members: Vec<usize>,
	pub member_of: Vec<usize>,
}
impl Group {
	pub fn new(parent_id: Option<usize>, name: Option<String>) -> Self {
		Self {
			parent_id,
			name,
			selector: None,
			class_names: Vec::new(),

			properties: Properties::default(),
			elements: Vec::new(),
			classes: HashMap::new(),
			listeners: Vec::new(),

			members: Vec::new(),
			member_of: Vec::new(),
		}
	}
}

pub struct Page {
	pub title: String,
	pub route: String,
	pub styles: CssRules,
	pub root_id: usize,
}

#[derive(Default)]
pub struct Semantics {
	pub errors: Vec<&'static str>,
	pub warnings: Vec<&'static str>,

	pub pages: Vec<Page>,
	pub groups: Vec<Group>,
}
impl Semantics {
	pub fn _error(&mut self, message: &'static str) {
		self.errors.push(message);
	}

	pub fn _warning(&mut self, message: &'static str) {
		self.warnings.push(message);
	}
}

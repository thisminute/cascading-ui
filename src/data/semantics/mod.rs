pub mod properties;

use {
	self::properties::Properties,
	std::{collections::HashMap, error::Error, fmt},
};

#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "There is an error: {}", self.0)
	}
}
impl Error for MyError {}

pub struct Group {
	pub parent_id: Option<usize>,
	pub name: Option<String>,
	pub id: Option<String>,

	pub properties: Properties,
	pub elements: Vec<usize>,
	pub classes: HashMap<String, Vec<usize>>,
	pub listeners: Vec<usize>,

	pub members: Vec<usize>,
	pub member_of: Vec<usize>,
}
impl Group {
	pub fn new(parent_id: Option<usize>, name: Option<String>) -> Self {
		Self {
			parent_id,
			name,
			id: None,

			properties: Properties::default(),
			elements: Vec::new(),
			classes: HashMap::new(),
			listeners: Vec::new(),

			members: Vec::new(),
			member_of: Vec::new(),
		}
	}
}

pub struct Semantics {
	pub errors: Vec<&'static str>,
	pub warnings: Vec<&'static str>,

	pub pages: Vec<usize>,
	pub groups: Vec<Group>,
}
impl Semantics {
	pub fn new() -> Self {
		Self {
			errors: Vec::new(),
			warnings: Vec::new(),

			pages: Vec::new(),
			groups: Vec::new(),
		}
	}

	pub fn _error(&mut self, message: &'static str) {
		self.errors.push(message);
	}

	pub fn _warning(&mut self, message: &'static str) {
		self.warnings.push(message);
	}
}

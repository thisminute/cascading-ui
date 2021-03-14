pub mod event;
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
	pub class_names: Vec<String>,

	pub members: Vec<usize>,
	pub properties: Properties,
	pub elements: Vec<usize>,

	pub member_of: Vec<usize>,
	pub scoped_groups: HashMap<String, Vec<usize>>,
}

pub struct Semantics {
	pub only_header_wasm: bool,
	pub bindgen: bool,

	pub errors: Vec<&'static str>,
	pub warnings: Vec<&'static str>,

	pub pages: Vec<usize>,
	pub groups: Vec<Group>,
}
impl Semantics {
	pub fn new(bindgen: bool) -> Self {
		Self {
			only_header_wasm: false,
			bindgen,

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

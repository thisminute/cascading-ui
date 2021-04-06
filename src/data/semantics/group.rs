use {
	data::semantics::properties::{CwlProperty, Properties},
	std::collections::HashMap,
};

#[derive(Clone, Debug)]
pub struct Group {
	pub name: Option<String>,
	pub selector: Option<String>,
	pub class_names: Vec<String>,
	pub r#static: bool,

	pub properties: Properties,
	pub elements: Vec<usize>,
	pub classes: HashMap<String, Vec<usize>>,
	pub listeners: Vec<usize>,

	pub members: Vec<usize>,
	pub member_of: Vec<usize>,
}
impl Group {
	pub fn new(name: Option<String>, r#static: bool) -> Self {
		Self {
			name,
			selector: None,
			class_names: Vec::new(),
			r#static,

			properties: Properties::default(),
			elements: Vec::new(),
			classes: HashMap::new(),
			listeners: Vec::new(),

			members: Vec::new(),
			member_of: Vec::new(),
		}
	}

	pub fn tag(&self) -> &'static str {
		match self.properties.cwl.get(&CwlProperty::Link) {
			Some(_) => "a",
			None => "div",
		}
	}
}

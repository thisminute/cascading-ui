use {
	data::semantics::properties::{CwlProperty, Properties},
	std::collections::HashMap,
};

#[derive(Clone, Debug)]
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

	pub fn tag(&self) -> &'static str {
		match self.properties.cwl.get(&CwlProperty::Link) {
			Some(_) => "a",
			None => "div",
		}
	}
}

use {
	data::semantics::properties::{CwlProperty, Properties},
	std::collections::HashMap,
};

#[derive(Clone, Debug)]
pub struct Group {
	pub name: Option<String>,
	pub selector: Option<String>,
	pub class_names: Vec<String>,
	pub listener_scope: Option<usize>,

	pub properties: Properties,
	pub elements: Vec<usize>,
	pub classes: HashMap<String, Vec<usize>>,
	pub listeners: Vec<usize>,

	pub members: Vec<usize>,
	pub member_of: Vec<usize>,
}
impl Group {
	pub fn new(name: Option<String>, listener_scope: Option<usize>) -> Self {
		Self {
			name,
			selector: None,
			class_names: Vec::new(),
			listener_scope,

			properties: Properties::default(),
			elements: Vec::new(),
			classes: HashMap::new(),
			listeners: Vec::new(),

			members: Vec::new(),
			member_of: Vec::new(),
		}
	}

	pub fn class_to_new_static_element(&mut self, source_id: usize) -> Self {
		Group {
			name: Some(
				self.name
					.clone()
					.expect("should never try to make an instance of a class with no name"),
			),
			selector: None,
			class_names: Vec::new(),
			listener_scope: None,

			properties: Properties {
				cwl: self.properties.cwl.clone(),
				css: HashMap::new(),
				page: HashMap::new(),
			},
			elements: self.elements.clone(),
			classes: HashMap::new(),
			listeners: self.listeners.clone(),

			members: Vec::new(),
			member_of: vec![source_id],
		}
	}

	pub fn tag(&self) -> &'static str {
		match self.properties.cwl.get(&CwlProperty::Link) {
			Some(_) => "a",
			None => "div",
		}
	}

	pub fn is_static(&self) -> bool {
		self.listener_scope.is_none()
	}
}

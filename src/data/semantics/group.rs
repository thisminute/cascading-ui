use {
	super::{properties::Property, Value},
	std::collections::HashMap,
};

#[derive(Clone)]
pub struct Group {
	pub name: Option<String>,
	pub listener_scope: Option<usize>,

	pub elements: Vec<usize>,
	pub classes: HashMap<String, Vec<usize>>,
	pub listeners: Vec<usize>,
	pub properties: HashMap<Property, Value>,
	pub variables: HashMap<String, usize>,

	// for element groups
	pub tag: &'static str,
	pub member_of: Vec<usize>,
	pub class_names: Vec<String>,

	// for class groups
	pub selector: Option<String>,
	pub members: Vec<usize>,
	pub has_css_properties: bool,
	pub is_dynamic: bool,
}
impl Group {
	pub fn new(
		name: Option<String>,
		listener_scope: Option<usize>,
		variables: HashMap<String, usize>,
	) -> Self {
		Self {
			name,
			listener_scope,

			elements: Vec::new(),
			classes: HashMap::new(),
			listeners: Vec::new(),
			properties: HashMap::new(),
			variables,

			tag: "div",
			member_of: Vec::new(),
			class_names: Vec::new(),

			selector: None,
			members: Vec::new(),
			has_css_properties: false,
			is_dynamic: false,
		}
	}

	pub fn class_to_new_compiled_element(&mut self, source_id: usize) -> Self {
		Group {
			name: Some(
				self.name
					.clone()
					.expect("should never try to make an instance of a class with no name"),
			),
			listener_scope: None,

			elements: self.elements.clone(),
			classes: HashMap::new(),
			listeners: self.listeners.clone(),
			properties: self.properties.clone(),
			variables: HashMap::new(),

			tag: "div",
			member_of: vec![source_id],
			class_names: Vec::new(),

			selector: None,
			members: Vec::new(),
			has_css_properties: self.has_css_properties,
			is_dynamic: false,
		}
	}

	pub fn is_compiled(&self) -> bool {
		self.listener_scope.is_none()
	}
}

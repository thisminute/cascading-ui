use {
	data::semantics::{
		properties::Properties,
		{Group, Semantics},
	},
	std::collections::HashMap,
};

impl Semantics {
	fn create_element_from_group(&mut self, source_id: usize, parent_id: usize) {
		let element_id = self.groups.len();
		eprintln!(" Creating new element group {}", element_id);
		let source = &mut self.groups[source_id];
		let element = Group {
			name: Some(
				source
					.name
					.clone()
					.expect("should never try to make an instance of a class with no name"),
			),
			selector: None,
			class_names: Vec::new(),
			r#static: true,

			properties: Properties {
				cwl: source.properties.cwl.clone(),
				css: HashMap::new(),
				page: HashMap::new(),
			},
			elements: source.elements.clone(),
			classes: source.classes.clone(),
			listeners: source.listeners.clone(),

			members: Vec::new(),
			member_of: vec![source_id],
		};
		source.members.push(element_id);
		self.groups.push(element);
		self.groups[parent_id].elements.push(element_id);
	}

	pub fn cascade(&mut self, source_id: usize, target_id: usize) {
		eprintln!(
			"Cascading from group {} into group {}",
			source_id, target_id
		);
		if source_id == target_id {
			panic!("the build process should never try to cascade a group into itself")
		}

		for (property, value) in self.groups[source_id].properties.cwl.clone() {
			eprintln!(" Cascading cwl property {:?}:{}", property, value);
			self.groups[target_id]
				.properties
				.cwl
				.entry(property)
				.or_insert(value.clone());
		}
		for (name, class_ids) in self.groups[source_id].classes.clone() {
			for class_id in class_ids {
				eprintln!(" Cascading scoped class with name {}", name);
				self.groups[target_id]
					.classes
					.entry(name.clone())
					.or_default()
					.push(class_id);
			}
		}
		for listener_id in self.groups[source_id].listeners.clone() {
			eprintln!(
				" Cascading scoped listener {} with properties {:?}",
				listener_id, self.groups[listener_id].properties
			);
			self.groups[target_id].listeners.push(listener_id);
		}

		if self.groups[source_id].elements.len() > 0 {
			if self.groups[target_id].elements.len() > 0 {
				panic!("Source and target group both have elements; their ordering cannot be determined")
			}
			for source_id in self.groups[source_id].elements.clone() {
				eprintln!(
					" Cascading element with name {}",
					self.groups[source_id].name.clone().unwrap()
				);
				self.create_element_from_group(source_id, target_id);
			}
		}
	}
}

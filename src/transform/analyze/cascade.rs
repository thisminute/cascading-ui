use {
	data::semantics::{
		properties::{CssProperties, Properties},
		Group, Semantics,
	},
	misc::id_gen::id_gen,
	std::collections::HashMap,
};

impl Semantics {
	fn create_group_from_group(&mut self, source_id: usize, parent_id: usize) {
		let element_id = self.groups.len();
		let group = &mut self.groups[source_id];
		let identifier = group
			.name
			.clone()
			.expect("should never try to make an instance of a class with no name");
		let element = Group {
			parent_id: Some(parent_id),
			name: Some(identifier),
			selector: None,
			class_names: Vec::new(),

			properties: Properties {
				cwl: group.properties.cwl.clone(),
				css: HashMap::new(),
				page: HashMap::new(),
			},
			elements: group.elements.clone(),
			classes: group.classes.clone(),
			listeners: group.listeners.clone(),

			members: Vec::new(),
			member_of: vec![source_id],
		};
		group.members.push(element_id);
		self.groups[parent_id].elements.push(element_id);
		self.groups.push(element);
	}

	pub fn cascade(&mut self, source_id: usize, target_id: usize) {
		eprintln!(
			"Cascading from group {} into group {}",
			source_id, target_id
		);
		if source_id == target_id {
			panic!("the build process should never try to cascade a group into itself")
		}

		if self.groups[source_id].elements.len() > 0 {
			if self.groups[target_id].elements.len() > 0 {
				panic!("Source and target group both have elements; their ordering cannot be determined")
			}
			for source_id in self.groups[source_id].elements.clone() {
				self.create_group_from_group(source_id, target_id);
			}
		}

		for (property, value) in self.groups[source_id].properties.cwl.clone() {
			eprintln!(" Cascading cwl property {:?}:{}", property, value);
			self.groups[target_id]
				.properties
				.cwl
				.entry(property)
				.or_insert(value.clone());
		}
		for _ in &self.groups[source_id].properties.page {
			panic!("page properties should never be cascaded");
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
		for (_, listener_id) in self.groups[source_id].listeners.clone() {
			eprintln!(" Cascading scoped listener {}", listener_id);
			self.groups[target_id]
				.listeners
				.push((id_gen(), listener_id));
		}
	}
}

impl Group {
	pub fn cascade_css(&mut self, source_css: CssProperties) {
		for (property, value) in source_css {
			eprintln!(" Cascading css property {:?}:{}", property, value);
			self.properties.css.entry(property).or_insert(value);
		}
	}
}

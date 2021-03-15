use {
	data::semantics::{properties::Properties, Group},
	std::collections::HashMap,
};

type Groups = Vec<Group>;

pub trait Cascade {
	fn create_group_from_group(&mut self, source_id: usize, parent_id: usize);
	fn cascade(&mut self, source_id: usize, target_id: usize);
	fn cascade_css(&mut self, source_id: usize, target_id: usize);
}

impl Cascade for Groups {
	fn create_group_from_group(&mut self, source_id: usize, parent_id: usize) {
		let element_id = self.len();
		let group = &mut self[source_id];
		let identifier = group.name.clone().unwrap();
		let element = Group {
			parent_id: Some(parent_id),
			name: Some(identifier),
			id: None,

			properties: Properties {
				cwl: group.properties.cwl.clone(),
				css: HashMap::new(),
				page: HashMap::new(),
			},
			elements: group.elements.clone(),
			classes: group.classes.clone(),

			members: Vec::new(),
			member_of: vec![source_id],
		};
		group.members.push(element_id);
		self[parent_id].elements.push(element_id);
		self.push(element);
	}

	fn cascade(&mut self, source_id: usize, target_id: usize) {
		eprintln!(
			"Cascading from group {} into group {}",
			source_id, target_id
		);
		if source_id == target_id {
			return;
		}

		if self[source_id].elements.len() > 0 {
			if self[target_id].elements.len() > 0 {
				panic!("Source and target group both have elements; their ordering cannot be determined")
			}
			for source_id in self[source_id].elements.clone() {
				self.create_group_from_group(source_id, target_id);
			}
		}

		for (&property, value) in &self[source_id].properties.cwl.clone() {
			eprintln!(" Cascading cwl property {:?}:{}", property, value);
			self[target_id]
				.properties
				.cwl
				.entry(property)
				.or_insert(value.clone());
		}
		for _ in &self[source_id].properties.page {
			panic!("page properties should never be cascaded");
		}
		for (name, class_ids) in &self[source_id].classes.clone() {
			for &class_id in class_ids {
				eprintln!(" Cascading scoped group with name {}", name);
				let classes = self[target_id].classes.entry(name.clone()).or_default();
				classes.push(class_id);
			}
		}
	}

	fn cascade_css(&mut self, source_id: usize, target_id: usize) {
		for (&property, value) in &self[source_id].properties.css.clone() {
			eprintln!(" Cascading css property {:?}:{}", property, value);
			self[target_id]
				.properties
				.css
				.entry(property)
				.or_insert(value.clone());
		}
	}
}

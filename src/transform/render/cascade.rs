use data::semantics::Semantics;

impl Semantics {
	fn create_element_from_group(&mut self, source_id: usize, parent_id: usize) {
		let element_id = self.groups.len();
		eprintln!(" Creating new element group {}", element_id);
		let source = &mut self.groups[source_id];
		let element = source.class_to_new_static_element(source_id);
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

		if self.groups[source_id].is_static() {
			for (property, value) in self.groups[source_id].properties.cwl.clone() {
				eprintln!(" Cascading cwl property {:?}:{}", property, value);
				self.groups[target_id]
					.properties
					.cwl
					.entry(property)
					.or_insert(value.clone());
			}
			for listener_id in self.groups[source_id].listeners.clone() {
				eprintln!(
					" Cascading scoped listener {} with properties {:?}",
					listener_id, self.groups[listener_id].properties
				);
				self.groups[target_id].listeners.push(listener_id);
			}
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

		if self.groups[source_id].is_static()
			&& self.groups[target_id].is_static()
			&& self.groups[source_id].elements.len() > 0
			&& self.groups[target_id].elements.len() > 0
		{
			panic!("Source and target group specify different contents for the same element")
		}

		if self.groups[source_id].elements.len() > 0 {
			for source_id in self.groups[source_id].elements.clone() {
				eprintln!(
					" Cascading element with name {}",
					self.groups[source_id].name.clone().unwrap()
				);
				if self.groups[source_id].is_static() {
					self.create_element_from_group(source_id, target_id);
				} else {
					self.groups[target_id].elements.push(source_id);
				}
			}
		}
	}
}

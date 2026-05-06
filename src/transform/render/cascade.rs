use {crate::data::semantics::Semantics, crate::misc::id_gen::generate_mutable_id};

impl Semantics {
	fn create_element_from_group(&mut self, source_id: usize, parent_id: usize) {
		let element_id = self.groups.len();
		log::debug!(" Creating new element group {}", element_id);
		let source = &mut self.groups[source_id];
		let element = source.class_to_new_compiled_element(source_id);
		source.members.push(element_id);
		self.groups.push(element);
		self.groups[parent_id].elements.push(element_id);
	}

	pub fn cascade(&mut self, source_id: usize, target_id: usize, virtual_: bool) {
		log::debug!(
			"Cascading from group {} into group {}",
			source_id,
			target_id
		);
		if source_id == target_id {
			panic!("the build process should never try to cascade a group into itself")
		}

		if !virtual_ {
			for (property, value) in self.groups[source_id].properties.clone() {
				log::debug!(" Cascading cui property {:?}:{:?}", property, value);
				self.groups[target_id]
					.properties
					.entry(property)
					.or_insert(value);
			}

			for (name, value) in self.groups[source_id].variables.clone() {
				log::debug!(" Cascading variable '{}': {:?}", name, value);
				self.groups[target_id]
					.variables
					.entry(name)
					.or_insert(value);
			}

			for (name, value) in self.groups[source_id].assignments.clone() {
				log::debug!(" Cascading assignment '{}': {:?}", name, value);
				self.groups[target_id]
					.assignments
					.entry(name)
					.or_insert(value);
			}
		} else {
			// Link declarations from source to declarations in target
			for (name, &source_variable_id) in &self.groups[source_id].variables {
				if let Some(&target_variable_id) = self.groups[target_id].variables.get(name) {
					log::debug!(
						" Adding mutable flag to variable '{}' with id {}",
						name,
						target_variable_id
					);
					let mutable_id = generate_mutable_id();
					self.variables[source_variable_id] = (
						self.variables[source_variable_id].0.clone(),
						Some(mutable_id),
					);
					self.variables[target_variable_id] = (
						self.variables[target_variable_id].0.clone(),
						Some(mutable_id),
					);
				}
			}
			// Link assignments from source to declarations in target
			for (name, &source_variable_id) in &self.groups[source_id].assignments {
				if let Some(&target_variable_id) = self.groups[target_id].variables.get(name) {
					log::debug!(
						" Adding mutable flag to assignment '{}' with id {}",
						name,
						target_variable_id
					);
					let mutable_id = self.variables[target_variable_id]
						.1
						.unwrap_or_else(generate_mutable_id);
					self.variables[source_variable_id] = (
						self.variables[source_variable_id].0.clone(),
						Some(mutable_id),
					);
					self.variables[target_variable_id] = (
						self.variables[target_variable_id].0.clone(),
						Some(mutable_id),
					);
				}
			}
		}

		for listener_id in self.groups[source_id].listeners.clone() {
			log::debug!(" Cascading scoped listener {}", listener_id);
			self.groups[target_id].listeners.push(listener_id);
			self.cascade(listener_id, target_id, true);
		}

		for (name, class_ids) in self.groups[source_id].classes.clone() {
			for class_id in class_ids {
				log::debug!(" Cascading scoped class {} with name {}", class_id, name);
				self.groups[target_id]
					.classes
					.entry(name.clone())
					.or_default()
					.push(class_id);
			}
		}

		if (self.groups[source_id].listener_scope == self.groups[target_id].listener_scope)
			&& !self.groups[source_id].elements.is_empty()
			&& !self.groups[target_id].elements.is_empty()
		{
			panic!("Source and target group specify different contents for the same element")
		}

		for element_id in self.groups[source_id].elements.clone() {
			log::debug!(
				" Cascading element with name {:?}",
				self.groups[element_id].name
			);
			if self.groups[element_id].is_compiled() {
				self.create_element_from_group(element_id, target_id);
			} else {
				self.groups[target_id].elements.push(element_id);
			}
		}
	}
}

use {data::semantics::Semantics, misc::id_gen::id_gen};

impl Semantics {
	pub fn render_element(&mut self, element_id: usize, ancestors: &mut Vec<usize>) {
		eprintln!("Rendering element {}", element_id);

		for ancestor_id in ancestors.clone() {
			eprintln!("Looking at ancestor: {}", ancestor_id);
			for class_id in self.groups[ancestor_id]
				.classes
				.get(
					&self.groups[element_id]
						.name
						.clone()
						.expect("element to have a name"),
				)
				.unwrap_or(&Vec::new())
				.clone()
			{
				eprintln!("Adding element {} to class {}", element_id, class_id);

				self.groups[class_id].members.push(element_id);
				self.groups[element_id].member_of.push(class_id);
			}
		}

		for class_id in self.groups[element_id].member_of.clone() {
			self.cascade(class_id, element_id);

			let selector = self.groups[class_id]
				.selector
				.get_or_insert_with(id_gen)
				.clone();
			if self.groups[element_id].r#static && !self.groups[class_id].properties.css.is_empty()
			{
				self.styles.insert(
					format!(".{}", selector),
					self.groups[class_id].properties.css.clone(),
				);
			}
			self.groups[element_id].class_names.push(selector);
		}

		for listener_id in self.groups[element_id].listeners.clone() {
			self.render_listener(listener_id, ancestors);
		}

		ancestors.push(element_id);
		for element_id in self.groups[element_id].elements.clone() {
			self.render_element(element_id, ancestors);
		}
		ancestors.pop();
	}
}

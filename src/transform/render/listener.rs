use {data::semantics::Semantics, misc::id_gen::id_gen};

impl Semantics {
	pub fn render_listener(&mut self, listener_id: usize, ancestors: &mut Vec<usize>) {
		eprintln!("Rendering listener {}", listener_id);

		for ancestor_id in ancestors.clone() {
			eprintln!("Looking at ancestor: {}", ancestor_id);
			for class_id in self.groups[ancestor_id]
				.classes
				.get(
					&self.groups[listener_id]
						.name
						.clone()
						.expect("element to have a name"),
				)
				.unwrap_or(&Vec::new())
				.clone()
			{
				eprintln!("Adding element {} to class {}", listener_id, class_id);
				self.groups[class_id].members.push(listener_id);
				self.groups[listener_id].member_of.push(class_id);
			}
		}

		for class_id in self.groups[listener_id].member_of.clone() {
			self.cascade(class_id, listener_id);

			let selector = self.groups[class_id]
				.selector
				.get_or_insert_with(id_gen)
				.clone();
			if self.groups[listener_id].r#static && !self.groups[class_id].properties.css.is_empty()
			{
				self.styles.insert(
					format!(".{}", selector),
					self.groups[listener_id].properties.css.clone(),
				);
			}
			self.groups[listener_id].class_names.push(selector);
		}

		ancestors.push(listener_id);
		for listener_id in self.groups[listener_id].elements.clone() {
			self.render_element(listener_id, ancestors);
		}
		ancestors.pop();
	}
}

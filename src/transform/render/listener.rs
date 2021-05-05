use {data::semantics::Semantics, misc::id_gen::id_gen};

impl Semantics {
	pub fn render_listener(&mut self, listener_id: usize, ancestors: &mut Vec<usize>) {
		eprintln!("Rendering listener {}", listener_id);

		let &parent_id = ancestors.last().unwrap();
		eprintln!(" Attaching to parent: {}", parent_id);
		for class_id in self.groups[parent_id]
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
			self.groups[class_id].members.push(listener_id);
			self.groups[listener_id].member_of.push(class_id);
		}

		for class_id in self.groups[listener_id].member_of.clone() {
			self.cascade(class_id, listener_id);

			let selector = self.groups[class_id]
				.selector
				.get_or_insert_with(id_gen)
				.clone();
			if self.groups[listener_id].is_static()
				&& !self.groups[class_id].properties.css.is_empty()
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

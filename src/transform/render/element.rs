use {data::semantics::Semantics, misc::id_gen::id_gen};

impl Semantics {
	pub fn render_element(&mut self, element_id: usize, ancestors: &mut Vec<usize>) {
		log::debug!("Rendering element {}", element_id);

		for ancestor_id in ancestors.clone() {
			log::debug!(" Looking at ancestor: {}", ancestor_id);
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
				log::debug!("  Attaching class {} to element {}", class_id, element_id);
				self.groups[class_id].members.push(element_id);
				self.groups[element_id].member_of.push(class_id);
			}
		}

		for listener_id in self.groups[element_id].listeners.clone() {
			log::debug!(
				" Attaching listener {} to element {}",
				listener_id,
				element_id,
			);
			self.cascade(listener_id, element_id);
		}

		for source_id in self.groups[element_id].member_of.clone() {
			// eventually we want to be able to uncomment this
			// if self.groups[element_id].is_static() && self.groups[source_id].properties.css.is_empty() {
			// 	continue;
			// }

			let selector = self.groups[source_id]
				.selector
				.get_or_insert_with(id_gen)
				.clone();
			if self.groups[source_id].is_static() {
				self.cascade(source_id, element_id);
				if !self.groups[source_id].properties.css.is_empty() {
					self.styles.insert(
						format!(".{}", selector),
						self.groups[source_id].properties.css.clone(),
					);
				}
			}
			self.groups[element_id].class_names.push(selector);
		}

		ancestors.push(element_id);
		for element_id in self.groups[element_id].elements.clone() {
			self.render_element(element_id, ancestors);
		}
		ancestors.pop();
	}
}

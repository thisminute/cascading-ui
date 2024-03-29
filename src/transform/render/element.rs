use {
	data::semantics::{
		properties::{CuiProperty, Property},
		Semantics,
	},
	misc::id_gen::generate_class_id,
};

impl Semantics {
	pub fn render_element(&mut self, element_id: usize, ancestors: &mut Vec<usize>) {
		log::debug!("Rendering element {}", element_id);

		// cascading can add more listeners, so this should ensure that the loop also iterates over those
		let mut last_idx = 0;
		while last_idx < self.groups[element_id].listeners.len() {
			let listener_id = self.groups[element_id].listeners[last_idx];
			log::debug!(
				" Attaching listener {} to element {}",
				listener_id,
				element_id,
			);
			self.cascade(listener_id, element_id, true);
			last_idx += 1;
		}

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
				if self.groups[element_id].listener_scope != self.groups[class_id].listener_scope {
					self.groups[class_id].is_dynamic = true;
				}
				self.groups[element_id].member_of.push(class_id);
				self.groups[class_id].members.push(element_id);
			}
		}

		for source_id in self.groups[element_id].member_of.clone() {
			self.cascade(
				source_id,
				element_id,
				self.groups[element_id].listener_scope != self.groups[source_id].listener_scope,
			);

			if !self.groups[source_id].is_dynamic && !self.groups[source_id].has_css_properties {
				continue;
			}

			let selector = self.groups[source_id]
				.selector
				.get_or_insert_with(generate_class_id)
				.clone();
			log::debug!("  Generated selector {} for group {}", selector, source_id);
			self.groups[element_id].class_names.push(selector.clone());

			if !self.groups[source_id].has_css_properties {
				continue;
			}

			self.styles.insert(
				format!(".{}", selector),
				(self.groups[source_id].properties.iter())
					.filter_map(|(property, value)| {
						if let Property::Css(property) = property {
							Some((property.clone(), value.clone()))
						} else {
							None
						}
					})
					.collect(),
			);
		}

		ancestors.push(element_id);
		self.render_values(element_id, ancestors);

		for element_id in self.groups[element_id].elements.clone() {
			self.render_element(element_id, ancestors);
		}
		ancestors.pop();

		self.groups[element_id].tag = if self.groups[element_id]
			.properties
			.contains_key(&Property::Cui(CuiProperty::Link))
		{
			"a"
		} else {
			"div"
		};

		log::debug!(" Removing virtual groups from element {}", element_id);
		let listener_scope = self.groups[element_id].listener_scope;
		self.groups[element_id].elements = self.groups[element_id]
			.elements
			.clone()
			.into_iter()
			.filter(|&group_id| listener_scope == self.groups[group_id].listener_scope)
			.collect();
		let mut classes = self.groups[element_id].classes.clone();
		for groups in &mut classes.values_mut() {
			groups.retain(|&group_id| listener_scope == self.groups[group_id].listener_scope)
		}
		self.groups[element_id].classes = classes;
		self.groups[element_id].listeners = self.groups[element_id]
			.listeners
			.clone()
			.into_iter()
			.filter(|&group_id| listener_scope == self.groups[group_id].listener_scope)
			.collect();
	}
}

mod cascade;
mod element;
mod value;

use {
	crate::data::semantics::{
		properties::{PageProperty, Property},
		Semantics, StaticValue, Value,
	},
	crate::misc::id_gen::generate_mutable_id,
};

impl Semantics {
	pub fn render(&mut self) {
		log::debug!("...Rendering...");
		for i in 0..self.pages.len() {
			let page_group_id = self.pages[i].root_id;
			log::debug!("Rendering page {}", page_group_id);
			// TODO: routes based on directory structure
			let ancestors = &mut Vec::new();
			let default = &Value::Static(StaticValue::String("".into()));
			let title = self.groups[page_group_id]
				.properties
				.get(&Property::Page(PageProperty::Title))
				.unwrap_or(default)
				.clone();
			self.pages[i].title = title;
			self.render_element(page_group_id, ancestors);
		}

		// Ensure persistent variables always have mutable_ids, even without
		// listener assignments. This makes them reactive so localStorage
		// values can be applied at startup.
		for (&var_id, _) in self.persistent_variables.clone().iter() {
			if self.variables[var_id].1.is_none() {
				self.variables[var_id].1 = Some(generate_mutable_id());
			}
		}

		self.mutable_count = generate_mutable_id();

		// Build mutable_id → localStorage key mapping for persistent variables
		for (&var_id, name) in self.persistent_variables.clone().iter() {
			if let (_, Some(mutable_id)) = &self.variables[var_id] {
				self.persistent_mutables
					.insert(*mutable_id, format!("cui:{}", name));
			}
		}
	}
}

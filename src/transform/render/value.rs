use {
	crate::data::semantics::{
		properties::{CuiProperty, Property},
		Semantics, StaticValue, Value,
	},
	crate::misc::id_gen::generate_mutable_id,
	proc_macro2::TokenStream,
	quote::ToTokens,
	std::fmt,
};

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Value::Static(value) => value.to_string(),
				Value::Variable(_, Some(value)) => value.to_string(),
				Value::Variable(variable_id, None) => format!("<{}>", variable_id),
				Value::UnrenderedVariable(_) => "@variable".to_string(),
				Value::ClassRef(name) => format!(".{}", name),
				Value::EventValue => "$value".to_string(),
			}
		)
	}
}

impl ToTokens for Value {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Value::ClassRef(_) | Value::EventValue => {}, // not runtime-literal values
			_ => self.to_string().to_tokens(tokens),
		}
	}
}

impl Semantics {
	pub fn render_values(&mut self, element_id: usize, ancestors: &[usize]) {
		for variable_id in self.groups[element_id]
			.variables
			.values()
			.cloned()
			.collect::<Vec<_>>()
		{
			let rendered = self.render_value(self.variables[variable_id].0.clone(), ancestors);
			self.variables[variable_id].0 = rendered;
		}

		for variable_id in self.groups[element_id]
			.assignments
			.values()
			.cloned()
			.collect::<Vec<_>>()
		{
			let rendered = self.render_value(self.variables[variable_id].0.clone(), ancestors);
			self.variables[variable_id].0 = rendered;
		}

		self.groups[element_id].properties = self.groups[element_id]
			.properties
			.clone()
			.into_iter()
			.map(|(identifier, value)| (identifier, self.render_value(value, ancestors)))
			.collect();
	}

	fn render_value(&mut self, value: Value, ancestors: &[usize]) -> Value {
		match value {
			Value::UnrenderedVariable(identifier) => {
				for &ancestor_id in ancestors {
					log::debug!(" Looking at ancestor: {}", ancestor_id);

					if let Some(&variable_id) = self.groups[ancestor_id].variables.get(&identifier)
					{
						return Value::Variable(
							variable_id,
							if let Value::Static(value) = &self.variables[variable_id].0 {
								Some(value.clone())
							} else {
								None
							},
						);
					}
				}
				// Built-in variable: $value reads the event target's value property
				if identifier == "value" {
					return Value::EventValue;
				}
				panic!("unable to render variable '{}' from ancestors", identifier)
			}
			Value::Variable(..) => value, // already rendered, idempotent
			value => value,
		}
	}

	/// Recursively resolve variable references in a dynamic subtree (listeners,
	/// their child elements, classes, and nested listeners). These groups aren't
	/// rendered as elements but their properties may reference ancestor variables.
	pub fn render_dynamic_subtree(&mut self, group_id: usize, ancestors: &[usize]) {
		self.render_values(group_id, ancestors);
		self.link_ancestor_assignments(group_id, ancestors);
		self.resolve_apply_targets(group_id, ancestors);
		for element_id in self.groups[group_id].elements.clone() {
			self.render_dynamic_subtree(element_id, ancestors);
		}
		for class_ids in self.groups[group_id].classes.clone().values().cloned().collect::<Vec<_>>() {
			for class_id in class_ids {
				self.render_dynamic_subtree(class_id, ancestors);
			}
		}
		for listener_id in self.groups[group_id].listeners.clone() {
			self.render_dynamic_subtree(listener_id, ancestors);
		}
	}

	/// Link assignment variables (bare `$var: value` without `let`) in a group
	/// to matching declared variables in ancestor scopes. This creates the
	/// mutable variable link needed for cross-element reactivity.
	fn link_ancestor_assignments(&mut self, group_id: usize, ancestors: &[usize]) {
		let assignments = self.groups[group_id].assignments.clone();
		for (name, variable_id) in &assignments {
			// Search ancestors (nearest first) for a declared variable with this name
			for &ancestor_id in ancestors.iter().rev() {
				if let Some(&ancestor_var_id) = self.groups[ancestor_id].variables.get(name) {
					if ancestor_var_id == *variable_id {
						continue; // Don't link to self
					}

					// Reuse ancestor's mutable_id or generate a new one
					let mutable_id = self.variables[ancestor_var_id]
						.1
						.unwrap_or_else(generate_mutable_id);

					self.variables[*variable_id] = (
						self.variables[*variable_id].0.clone(),
						Some(mutable_id),
					);
					self.variables[ancestor_var_id] = (
						self.variables[ancestor_var_id].0.clone(),
						Some(mutable_id),
					);

					log::debug!(
						"Linked assignment '{}' (var {}) to ancestor declaration (var {}) with mutable_id {}",
						name, variable_id, ancestor_var_id, mutable_id
					);
					break;
				}
			}
		}
	}

	/// When a group has `apply: .class_name`, find the target class group in
	/// ancestor scopes and record it in `apply_targets`. Also render the target
	/// class's dynamic subtree so its variable references and nested apply
	/// targets are resolved.
	fn resolve_apply_targets(&mut self, group_id: usize, ancestors: &[usize]) {
		let apply_prop = self.groups[group_id]
			.properties
			.get(&Property::Cui(CuiProperty::Apply))
			.cloned();

		if let Some(Value::ClassRef(class_name)) = apply_prop {
			// Already resolved — don't re-render (prevents infinite loops in
			// circular apply references like A → B → A)
			if self.apply_targets.contains_key(&class_name) {
				return;
			}

			// Search ancestors (nearest first) for a class with this name
			for &ancestor_id in ancestors.iter().rev() {
				if let Some(class_ids) = self.groups[ancestor_id].classes.get(&class_name) {
					if let Some(&class_id) = class_ids.first() {
						log::debug!(
							"Resolved apply target '{}' to class group {}",
							class_name,
							class_id
						);
						self.apply_targets.insert(class_name.clone(), class_id);

						// Render the target class's dynamic subtree so its
						// listeners have their variables resolved and any nested
						// apply: references are also resolved.
						self.render_dynamic_subtree(class_id, ancestors);
						break;
					}
				}
			}
		}
	}

	pub fn get_static(&self, value: &Value) -> StaticValue {
		match value {
			Value::Static(value) => value.clone(),
			&Value::Variable(variable_id, None) => self.get_static(&self.variables[variable_id].0),
			Value::Variable(_, Some(value)) => value.clone(),
			Value::UnrenderedVariable(_) => {
				panic!("cannot get static value of unrendered variable")
			}
			Value::ClassRef(name) => StaticValue::String(format!(".{}", name)),
			Value::EventValue => StaticValue::String(String::new()),
		}
	}
}

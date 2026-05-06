use {
	crate::data::semantics::{Semantics, StaticValue, Value},
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
			}
		)
	}
}

impl ToTokens for Value {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.to_string().to_tokens(tokens)
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
				panic!("unable to render variable from ancestors")
			}
			Value::Variable(..) => panic!("already rendered variable"),
			value => value,
		}
	}

	/// Recursively resolve variable references in a dynamic subtree (listeners,
	/// their child elements, classes, and nested listeners). These groups aren't
	/// rendered as elements but their properties may reference ancestor variables.
	pub fn render_dynamic_subtree(&mut self, group_id: usize, ancestors: &[usize]) {
		self.render_values(group_id, ancestors);
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

	pub fn get_static(&self, value: &Value) -> StaticValue {
		match value {
			Value::Static(value) => value.clone(),
			&Value::Variable(variable_id, None) => self.get_static(&self.variables[variable_id].0),
			Value::Variable(_, Some(value)) => value.clone(),
			Value::UnrenderedVariable(_) => {
				panic!("cannot get static value of unrendered variable")
			}
		}
	}
}

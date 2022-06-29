use data::semantics::{Semantics, StaticValue, Value};

impl Value {
	pub fn get_static(&self) -> &StaticValue {
		match &self {
			Value::Variable(_, Some(value), _) => value,
			Value::Static(value) => value,
			_ => panic!("AAA"),
		}
	}

	pub fn get_string(&self) -> String {
		match self.get_static() {
			StaticValue::String(value) => value.clone(),
			_ => panic!("aaa"),
		}
	}
}

impl Semantics {
	pub fn render_values(&mut self, element_id: usize, ancestors: &[usize]) {
		self.groups[element_id].variables = self.groups[element_id]
			.variables
			.clone()
			.into_iter()
			.map(|(identifier, value)| (identifier, self.render_value(value, ancestors)))
			.collect();

		self.groups[element_id].properties.cui = self.groups[element_id]
			.properties
			.cui
			.clone()
			.into_iter()
			.map(|(identifier, value)| (identifier, self.render_value(value, ancestors)))
			.collect();

		self.groups[element_id].properties.css = self.groups[element_id]
			.properties
			.css
			.clone()
			.into_iter()
			.map(|(identifier, value)| (identifier, self.render_value(value, ancestors)))
			.collect();

		self.groups[element_id].properties.page = self.groups[element_id]
			.properties
			.page
			.clone()
			.into_iter()
			.map(|(identifier, value)| (identifier, self.render_value(value, ancestors)))
			.collect();
	}

	fn render_value(&self, value: Value, ancestors: &[usize]) -> Value {
		match value {
			Value::Variable(identifier, None, id) => {
				for &ancestor_id in ancestors {
					log::debug!(" Looking at ancestor: {}", ancestor_id);

					if let Some(value) = self.groups[ancestor_id]
						.variables
						.get(&identifier.to_string())
					{
						return Value::Variable(identifier, Some(value.get_static().clone()), id);
					}
				}
				panic!("unable to render variable from ancestors")
			}
			Value::Variable(_, Some(_), _) => panic!("already rendered variable"),
			value => value,
		}
	}
}

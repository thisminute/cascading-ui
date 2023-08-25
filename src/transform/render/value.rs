use data::semantics::{Semantics, Value};

impl Semantics {
	pub fn render_values(&mut self, group_id: usize, ancestors: &[usize]) {
		for variable_id in self.groups[group_id]
			.variables
			.values()
			.cloned()
			.collect::<Vec<_>>()
		{
			self.render_value(self.variables[variable_id].0.clone(), ancestors);
		}

		self.groups[group_id].properties = self.groups[group_id]
			.properties
			.clone()
			.into_iter()
			.map(|(property, value)| (property, self.render_value(value, ancestors)))
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
}

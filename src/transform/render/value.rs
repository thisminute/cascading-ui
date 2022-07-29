use data::semantics::{Semantics, StaticValue, Value};
use {proc_macro2::TokenStream, quote::ToTokens, std::fmt};

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Value::Static(value) => value.to_string(),
				Value::Variable(_, Some(value), _) => value.to_string(),
				Value::Variable(_, None, _) => "@variable".to_string(),
			}
		)
	}
}

impl ToTokens for Value {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.to_string().to_tokens(tokens)
	}
}

impl Value {
	pub fn get_static(&self) -> &StaticValue {
		match &self {
			Value::Variable(_, Some(value), _) => value,
			Value::Static(value) => value,
			_ => panic!("AAA"),
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

		self.groups[element_id].properties = self.groups[element_id]
			.properties
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

use {
	data::{
		ast::Unit,
		semantics::{Semantics, StaticValue, Value},
	},
	proc_macro2::TokenStream,
	quote::{quote, ToTokens},
	std::fmt,
};

impl Semantics {
	pub fn initial_value(&self, value: &Value) -> TokenStream {
		match value {
			Value::Static(value) => quote! { #value },
			Value::Variable(_, Some(value)) => quote! { #value },
			&Value::Variable(variable_id, None) => {
				self.initial_value(&self.variables[variable_id].0)
			}
			Value::UnrenderedVariable(_) => {
				panic!("cannot get initial value of unrendered variable")
			}
		}
	}
	pub fn dynamic_value(&self, value: &Value) -> TokenStream {
		match &value {
			Value::Static(value) => quote! { #value },
			Value::Variable(variable_id, _) => match self.variables[*variable_id] {
				(_, Some(mutable_id)) => quote! { state[#mutable_id] },
				_ => self.initial_value(value),
			},
			Value::UnrenderedVariable(_) => {
				panic!("cannot get dynamic value of unrendered variable")
			}
		}
	}
}

impl ToTokens for StaticValue {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Self::Number(value, unit) => match unit {
				Unit::None => quote! { Value::Number(#value) },
				_ => panic!("at the disco"),
			},
			Self::String(value) => quote! { Value::String(#value) },
		}
		.to_tokens(tokens)
	}
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match &self {
				Self::Static(value) => value.to_string(),
				&Self::Variable(_, Some(value)) => value.to_string(),
				&Self::Variable(_, None) => panic!("cannot get initial value of this variable"),
				Self::UnrenderedVariable(_) => {
					panic!("cannot get string for unrendered variable")
				}
			}
		)
	}
}

impl fmt::Display for StaticValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::String(value) => value.clone(),
				Self::Number(value, unit) => format!("{}{}", value, unit),
				// StaticVaue::Color(r, g, b, a) => {
				// 	if *a > 0.999 {
				// 		format!("#{:X}{:X}{:X}", r, g, b)
				// 	} else {
				// 		format!("rgba({},{},{},{})", r, g, b, a)
				// 	}
				// }
			}
		)
	}
}

impl fmt::Display for Unit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::None => "",
				Self::Percent => "%",
				Self::Pixel => "px",
				// StaticVaue::Color(r, g, b, a) => {
				// 	if *a > 0.999 {
				// 		format!("#{:X}{:X}{:X}", r, g, b)
				// 	} else {
				// 		format!("rgba({},{},{},{})", r, g, b, a)
				// 	}
				// }
			}
		)
	}
}

use {proc_macro2::TokenStream, quote::ToTokens, std::fmt};

#[derive(Clone, Debug)]
pub enum Value {
	Static(StaticValue),
	Variable(String, Option<StaticValue>, Option<usize>),
}

#[derive(Clone, Debug)]
pub enum StaticValue {
	Number(i32),
	String(String),
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Value::Static(value) => match value {
					StaticValue::String(value) => value.clone(),
					StaticValue::Number(value) => value.to_string(),
				},
				Value::Variable(..) => "@variable".to_string(),
			}
		)
	}
}

impl ToTokens for Value {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.to_string().to_tokens(tokens)
	}
}

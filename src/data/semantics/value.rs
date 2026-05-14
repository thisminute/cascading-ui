use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
	Static(StaticValue),
	Variable(usize, Option<StaticValue>),
	UnrenderedVariable(String),
	ClassRef(String),
	Concat(Vec<Value>),
}

impl Value {
	/// If all parts of a Concat are static, collapse into a single Static string value.
	pub fn try_collapse(self) -> Self {
		match self {
			Value::Concat(parts) => {
				let parts: Vec<Value> = parts.into_iter().map(|p| p.try_collapse()).collect();
				if parts.iter().all(|p| matches!(p, Value::Static(_))) {
					let s: String = parts.iter().map(|p| p.to_string()).collect();
					Value::Static(StaticValue::String(s))
				} else {
					Value::Concat(parts)
				}
			}
			other => other,
		}
	}
}

#[derive(Clone, Debug)]
pub enum StaticValue {
	Number(i32),
	String(String),
	// Color(u8, u8, u8, f64),
}

impl fmt::Display for StaticValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				StaticValue::String(value) => value.clone(),
				StaticValue::Number(value) => value.to_string(),
				// StaticValue::Color(r, g, b, a) => {
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

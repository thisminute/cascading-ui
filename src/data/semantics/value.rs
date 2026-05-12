use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
	Static(StaticValue),
	Variable(usize, Option<StaticValue>),
	UnrenderedVariable(String),
	ClassRef(String),
}

#[derive(Clone, Debug)]
pub enum StaticValue {
	Number(i32),
	Float(f64),
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
				StaticValue::Float(value) => {
					// Format float without trailing zeros: 0.5 → "0.5", 1.0 → "1"
					let s = value.to_string();
					if s.ends_with(".0") {
						s[..s.len() - 2].to_string()
					} else {
						s
					}
				}
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

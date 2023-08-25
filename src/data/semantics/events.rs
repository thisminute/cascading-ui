use std::fmt;

#[derive(Clone, Debug)]
pub enum Event {
	Click,
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

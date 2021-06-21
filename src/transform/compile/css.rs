use data::semantics::properties::{CssProperties, CssRules};

pub trait Css {
	fn css(&self) -> String;
}

impl Css for Vec<CssRules> {
	fn css(&self) -> String {
		self
			.iter()
			.map(|rule| rule.css())
			.collect::<Vec<String>>()
			.join("")
	}
}

impl Css for CssRules {
	fn css(&self) -> String {
		self
			.iter()
			.map(|(selector, properties)| format!("{}{{{}}}", selector, properties.css()))
			.collect::<Vec<String>>()
			.join("")
	}
}

impl Css for CssProperties {
	fn css(&self) -> String {
		self
			.iter()
			.map(|(property, value)| format!("{}:{}", property, value))
			.collect::<Vec<String>>()
			.join(";")
	}
}

use crate::data::{CssProperties, CssProperty, CssRule};
pub trait Css {
	fn css(&self) -> String;
}

impl Css for Vec<CssRule> {
	fn css(&self) -> String {
		self.iter()
			.map(|rule| rule.css())
			.collect::<Vec<String>>()
			.join("")
	}
}

impl Css for CssRule {
	fn css(&self) -> String {
		format!("{}{{{}}}", self.selector, self.properties.css())
	}
}

impl Css for CssProperties {
	fn css(&self) -> String {
		[
			("background-color", self.get(&CssProperty::BackgroundColor)),
			("color", self.get(&CssProperty::Color)),
		]
		.iter()
		.filter(|(_, value)| value.is_some())
		.map(|(property, value)| format!("{}:{}", property, value.unwrap()))
		.collect::<Vec<String>>()
		.join(";")
	}
}

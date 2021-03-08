use crate::data::{CssProperties, CssProperty};
pub trait Css {
	fn css(&self) -> String;
}
// impl CssRule {
// 	pub fn css(&self) -> String {
// 		format!("{}{{{}}}", self.selector, self.properties.css())
// 	}
// }

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

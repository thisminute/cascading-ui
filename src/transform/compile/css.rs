use data::semantics::properties::{CssProperties, CssProperty, CssRules};

pub trait Css {
	fn css(&self) -> String;
}

impl Css for Vec<CssRules> {
	fn css(&self) -> String {
		self.iter()
			.map(|rule| rule.css())
			.collect::<Vec<String>>()
			.join("")
	}
}

impl Css for CssRules {
	fn css(&self) -> String {
		self.iter()
			.map(|(selector, properties)| format!("{}{{{}}}", selector, properties.css()))
			.collect::<Vec<String>>()
			.join("")
	}
}

impl Css for CssProperties {
	fn css(&self) -> String {
		self.iter()
			.map(|(property, value)| format!("{}:{}", property.css(), value))
			.collect::<Vec<String>>()
			.join(";")
	}
}

impl Css for CssProperty {
	fn css(&self) -> String {
		match self {
			Self::BackgroundColor => "background-color",
			Self::Color => "color",
			Self::Margin => "margin",
			Self::Padding => "padding",
			Self::Display => "display",
			Self::Position => "position",
			Self::Width => "width",
			Self::Height => "height",
		}
		.into()
	}
}

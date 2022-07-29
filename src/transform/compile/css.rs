use data::semantics::properties::{CssProperties, CssRules, Properties, Property};

pub trait Css {
	fn css(&self) -> String;
}

impl Css for Vec<CssRules> {
	fn css(&self) -> String {
		self.iter().map(|rule| rule.css()).collect::<String>()
	}
}

impl Css for CssRules {
	fn css(&self) -> String {
		self
			.iter()
			.map(|(selector, properties)| format!("{}{{{}}}", selector, properties.css()))
			.collect::<String>()
	}
}

impl Css for CssProperties {
	fn css(&self) -> String {
		self
			.iter()
			.map(|(property, value)| format!("{}:{}", property, value))
			.fold(String::with_capacity(self.len()), |mut a, b| {
				a.push_str(&b);
				a.push(';');
				a
			})
	}
}

impl Css for Properties {
	fn css(&self) -> String {
		self
			.iter()
			.filter_map(|(property, value)| {
				if let Property::Css(property) = property {
					Some(format!("{}:{}", property, value))
				} else {
					None
				}
			})
			.fold(String::with_capacity(self.len()), |mut a, b| {
				a.push_str(&b);
				a.push(';');
				a
			})
	}
}

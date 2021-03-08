use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum CssProperty {
	BackgroundColor,
	Color,
	Margin,
	Padding,
}
pub type CssProperties = HashMap<CssProperty, String>;

#[derive(Default, Clone, Debug)]
pub struct Properties {
	pub title: Option<String>,
	pub route: Option<String>,

	pub link: Option<String>,
	pub text: Option<String>,
	pub tooltip: Option<String>,

	pub css: CssProperties,
	pub image: Option<String>,
}

impl Properties {
	pub fn cascade(&mut self, properties: Self) {
		for (property, value) in properties.css {
			self.css.entry(property).or_insert(value);
		}
	}
}

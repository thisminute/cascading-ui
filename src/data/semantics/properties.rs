use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum CssProperty {
	BackgroundColor,
	Color,
	Margin,
	Padding,
}
pub type CssProperties = HashMap<CssProperty, String>;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum CwlProperty {
	Link,
	Text,
	Tooltip,
	Image,
}
pub type CwlProperties = HashMap<CwlProperty, String>;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum PageProperty {
	Title,
	Route,
}
pub type PageProperties = HashMap<PageProperty, String>;

#[derive(Default, Clone, Debug)]
pub struct Properties {
	pub page: PageProperties,
	pub css: CssProperties,
	pub cwl: CwlProperties,
}

impl Properties {
	pub fn cascade(&mut self, properties: &Self, cascade_css: bool) {
		for (property, value) in &properties.cwl {
			self.cwl.entry(*property).or_insert(value.clone());
		}
		for (property, value) in &properties.page {
			self.page.entry(*property).or_insert(value.clone());
		}
		if cascade_css {
			for (property, value) in &properties.css {
				self.css.entry(*property).or_insert(value.clone());
			}
		}
	}
}

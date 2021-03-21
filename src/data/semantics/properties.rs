use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum PageProperty {
	Title,
	Route,
}
pub type PageProperties = HashMap<PageProperty, String>;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum CssProperty {
	BackgroundColor,
	Color,
	Margin,
	Padding,
	Display,
	Position,
	Width,
	Height,
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

pub type CssRules = HashMap<String, CssProperties>;

#[derive(Default, Clone, Debug)]
pub struct Properties {
	pub page: PageProperties,
	pub css: CssProperties,
	pub cwl: CwlProperties,
}

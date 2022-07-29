mod group;
pub mod properties;
mod value;

pub use self::{
	group::Group,
	value::{StaticValue, Value},
};

use self::properties::CssRules;

pub struct Page {
	pub title: Value,
	pub route: &'static str,
	pub root_id: usize,
}

#[derive(Default)]
pub struct Semantics {
	pub errors: Vec<&'static str>,
	pub warnings: Vec<&'static str>,
	pub styles: CssRules,

	pub pages: Vec<Page>,
	pub groups: Vec<Group>,
	pub variables: Vec<Value>,
}

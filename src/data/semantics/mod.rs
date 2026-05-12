mod group;
pub mod properties;
mod value;

pub use self::{
	group::Group,
	value::{StaticValue, Value},
};

use {self::properties::CssRules, std::collections::HashMap};

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
	pub variables: Vec<(Value, Option<usize>)>,

	pub mutable_count: usize,

	/// Maps class names to group_ids for classes referenced by `apply:` properties
	pub apply_targets: HashMap<String, usize>,

	/// Maps variable_id → variable_name for variables marked with `persist`
	pub persistent_variables: HashMap<usize, String>,
	/// Maps mutable_id → localStorage key for persistent variables (built after render)
	pub persistent_mutables: HashMap<usize, String>,
}

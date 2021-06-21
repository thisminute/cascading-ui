mod group;
pub mod properties;

pub use self::group::Group;

use data::semantics::properties::CssRules;

pub struct Page {
	pub title: String,
	pub route: String,
	pub root_id: usize,
}

#[derive(Default)]
pub struct Semantics {
	pub errors: Vec<&'static str>,
	pub warnings: Vec<&'static str>,
	pub styles: CssRules,

	pub pages: Vec<Page>,
	pub groups: Vec<Group>,
}

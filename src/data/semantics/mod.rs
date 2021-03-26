mod group;
pub mod properties;

pub use self::group::Group;

use self::properties::CssRules;

pub struct Page {
	pub title: String,
	pub route: String,
	pub styles: CssRules,
	pub root_id: usize,
}

#[derive(Default)]
pub struct Semantics {
	pub errors: Vec<&'static str>,
	pub warnings: Vec<&'static str>,

	pub pages: Vec<Page>,
	pub groups: Vec<Group>,
}
impl Semantics {
	pub fn _error(&mut self, message: &'static str) {
		self.errors.push(message);
	}

	pub fn _warning(&mut self, message: &'static str) {
		self.warnings.push(message);
	}
}

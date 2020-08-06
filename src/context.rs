use crate::tokens::Prefix;

pub struct Ancestor {
	pub r#type: Prefix,
	pub string: String,
}

pub type Context = Vec<Ancestor>;

pub trait Info {
	fn is_root(&self) -> bool;
	fn is_static(&self) -> bool;
}

impl Info for Context {
	fn is_root(&self) -> bool {
		self.len() == 1
	}

	fn is_static(&self) -> bool {
		for ancestor in self {
			match ancestor.r#type {
				Prefix::Instance => return false,
				_ => {}
			}
		}
		true
	}
}

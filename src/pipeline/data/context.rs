use super::tokens::Prefix;

pub struct Ancestor {
	pub prefix: Prefix,
	pub identifier: String,
}

pub type Context = Vec<Ancestor>;

pub trait Info {
	fn is_root(&self) -> bool;
	fn is_static(&self) -> bool;
	fn to_string(&self) -> String;
}

impl Info for Context {
	fn is_root(&self) -> bool {
		self.len() == 1
	}

	fn is_static(&self) -> bool {
		self
			.iter()
			.all(|ancestor| Prefix::Instance == ancestor.prefix)
	}

	fn to_string(&self) -> String {
		self
			.iter()
			.map(|ancestor| &ancestor.identifier[..])
			.collect::<Vec<&str>>()
			.join("-")
	}
}

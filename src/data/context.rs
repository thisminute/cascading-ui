use super::tokens::Block;

// Blocks should live as long as the Document struct that owns them and outlives Contexts
#[derive(Clone)]
pub struct Context<'a> {
	pub block: &'a Block,
	pub is_static: bool,
	pub string: &'a str,
}

impl Context<'_> {
	// pub fn new(&self) -> Self {
	// 	Self {
	// 		block: None,
	// 		path: Vec::new(),
	// 		is_static: true,
	// 		string: "",
	// 	}
	// }

	pub fn is_root(&self) -> bool {
		true
	}

	// pub fn static_context(&self) -> String {
	// 	let mut static_ancestors = Vec::new();
	// 	for i in self.path {
	// 		let ancestor =
	// 		match ancestor {
	// 			 if *prefix == Prefix::Instance => {
	// 				static_ancestors.push(identifier.to_string().clone());
	// 			}
	// 			_ => break,
	// 		}
	// 	}
	// 	static_ancestors.join("-")
	// }
}

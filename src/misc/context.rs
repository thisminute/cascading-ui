use data::ast::Block;

// Blocks should live as long as the Document struct that owns them and which outlives Contexts
pub struct Context<'a> {
	pub block: &'a Block,
	pub path: Vec<usize>,
	pub root: usize,
	// pub events: Vec<Event>,
}

impl Context<'_> {
	pub fn is_root(&self) -> bool {
		self.block.identifier == "_"
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

use syn::{Expr, Ident};

// struct HyphenatedIdent {
// 	pub parts: Vec<Ident>,
// }

pub struct Document {
	pub root: Block,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Prefix {
	Page,
	Instance,
	Class,
	Action,
	Listener,
}

pub struct Block {
	pub prefix: Prefix,
	pub identifier: Ident,
	pub properties: Vec<Property>,
	pub blocks: Vec<Block>,
}

pub struct Property {
	pub property: Ident,
	pub value: Expr,
}

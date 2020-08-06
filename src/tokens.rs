use syn::{Expr, Ident};

// struct HyphenatedIdent {
// 	pub parts: Vec<Ident>,
// }

pub struct Website {
	pub document: Document,
}

pub struct Lib {}

pub struct Document {
	pub root: Block,
}

#[derive(Copy, Clone)]
pub enum Prefix {
	Instance,
	Class,
	Action,
	Listener,
}
pub struct Block {
	pub prefix: Prefix,
	pub identifier: Ident,
	pub rules: Vec<Rule>,
	pub blocks: Vec<Block>,
}

pub struct Rule {
	pub property: Ident,
	pub value: Expr,
}

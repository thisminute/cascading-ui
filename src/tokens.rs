use syn::{Expr, Ident};

#[derive(Debug)]
pub struct HyphenatedIdent {
	pub parts: Vec<Ident>,
}

#[derive(Debug)]
pub struct Rule {
	pub property: Ident,
	pub value: Expr,
}

#[derive(Debug)]
pub enum Prefix {
	Instance,
	Class,
	Action,
	Listener,
}

#[derive(Debug)]
pub struct Block {
	pub prefix: Prefix,
	pub identifier: Ident,
	pub rules: Vec<Rule>,
	pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Document {
	pub root: Block,
}

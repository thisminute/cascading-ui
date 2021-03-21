use syn::{Expr, Ident};

pub struct Document {
	pub root: Block,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Prefix {
	Element,
	Class,
	Action,
	Listener,
}

pub struct Block {
	pub prefix: Prefix,
	pub identifier: Ident,
	pub properties: Vec<Property>,
	pub elements: Vec<Block>,
	pub classes: Vec<Block>,
	pub listeners: Vec<Block>,
}

pub struct Property {
	pub property: Ident,
	pub value: Expr,
}

use {
	crate::meta::Meta,
	syn::{Expr, Ident},
};

#[derive(Debug)]
struct HyphenatedIdent {
	pub parts: Vec<Ident>,
}

pub struct Website<'a> {
	pub document: Document<'a>,
}

#[derive(Debug)]
pub struct Header {}

pub struct Document<'a> {
	pub meta: Meta<'a>,
	pub root: Block,
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
pub struct Rule {
	pub property: Ident,
	pub value: Expr,
}

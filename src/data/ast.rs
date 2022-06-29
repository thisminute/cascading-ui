use syn::Ident;

pub struct Document {
	pub root: Block,
}

#[derive(Debug)]
pub enum Prefix {
	Element,
	Class,
	Listener,
}

pub struct Block {
	pub prefix: Prefix,
	pub identifier: Ident,
	pub properties: Vec<Property>,
	pub elements: Vec<Block>,
	pub classes: Vec<Block>,
	pub listeners: Vec<Block>,
	pub variables: Vec<(String, Value)>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Variable(pub Ident);

#[derive(Clone, Debug)]
pub enum Value {
	Number(i32),
	String(String),
	Variable(Variable),
}

pub struct Property {
	pub property: Ident,
	pub value: Value,
}

pub struct Assignment {
	pub variable: Variable,
	pub value: Value,
}

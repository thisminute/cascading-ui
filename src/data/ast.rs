use syn::Ident;

pub struct Document {
	pub root: Block,
	pub keyframes: Vec<KeyframesBlock>,
}

#[derive(Clone)]
pub struct KeyframesBlock {
	pub name: String,
	pub steps: Vec<KeyframeStep>,
}

#[derive(Clone)]
pub struct KeyframeStep {
	pub selector: String,
	pub properties: Vec<Property>,
}

impl KeyframesBlock {
	pub fn to_css(&self) -> String {
		let steps: String = self
			.steps
			.iter()
			.map(|step| {
				let props: String = step
					.properties
					.iter()
					.map(|p| {
						let value = match &p.value {
							Value::String(s) => s.clone(),
							Value::Number(n) => n.to_string(),
							_ => String::new(),
						};
						format!("{}:{};", p.property, value)
					})
					.collect();
				format!("{}{{{}}} ", step.selector, props)
			})
			.collect();
		format!("@keyframes {}{{{}}}", self.name, steps)
	}
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
	pub keyframes: Vec<KeyframesBlock>,
	pub variables: Vec<(String, Value)>,
	pub assignments: Vec<(String, Value)>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Variable(pub Ident);

#[derive(Clone, Debug)]
pub enum Value {
	Number(i32),
	String(String),
	Variable(Variable),
	ClassRef(String),
}

#[derive(Clone)]
pub struct Property {
	pub property: String,
	pub value: Value,
}

pub struct Assignment {
	pub variable: Variable,
	pub value: Value,
}

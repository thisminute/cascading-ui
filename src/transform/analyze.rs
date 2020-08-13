use {
	data::{
		tokens::{Block, Document, Prefix, Rule},
		Context, Semantics,
	},
	syn::export::quote::quote,
};

pub trait Analyze {
	fn analyze(&self, semantics: &mut Semantics, context: &Context);
}

impl Analyze for Document {
	fn analyze(&self, semantics: &mut Semantics, context: &Context) {
		self.root.analyze(semantics, context);

		match &semantics.title {
			Some(_) => {}
			None => semantics.warning("you must set a title for the page"),
		}
		match &semantics.dom {
			Some(_) => {}
			None => semantics.warning("you must actually write a dom......."),
		}
	}
}

impl Analyze for Block {
	fn analyze(&self, semantics: &mut Semantics, context: &Context) {
		if context.is_static {
			match self.prefix {
				Prefix::Instance => {
					// semantics.element(context);
				}
				Prefix::Class => {}
				Prefix::Action => {}
				Prefix::Listener => {}
			}
		}

		let identifier = &self.identifier.to_string();
		let mut context_vec = Vec::new();
		if context.string.len() > 0 {
			context_vec.push(context.string);
			context_vec.push(identifier);
		} else if identifier != "_" {
			context_vec.push(identifier);
		}

		let context = Context {
			block: self,
			string: &context_vec.join("-"),
			is_static: context.is_static && self.prefix == Prefix::Instance,
			path: Vec::new(),
		};
		for rule in &self.rules {
			rule.analyze(semantics, &context);
		}
		for block in &self.blocks {
			block.analyze(semantics, &context);
		}
	}
}

impl Analyze for Rule {
	fn analyze(&self, semantics: &mut Semantics, context: &Context) {
		let property = &self.property.to_string()[..];
		let value = &self.value;

		match context.block.prefix {
			Prefix::Instance => match property {
				"title" if context.is_root() => match &semantics.title {
					Some(_title) => semantics.error("title property cannot be set more than once"),
					None => semantics.title = Some(quote! { #value }),
				},

				"text" => {}
				"link" => {}
				"tip" => {}
				_ => {}
			},
			Prefix::Class => {}
			Prefix::Action => {}
			Prefix::Listener => {}
		}
	}
}

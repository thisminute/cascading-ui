use {
	data::{
		tokens::{Block, Document, Prefix, Rule},
		Context, Semantics,
	},
	syn::export::ToTokens,
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
	}
}

impl Analyze for Block {
	fn analyze(&self, semantics: &mut Semantics, context: &Context) {
		let mut context_vec = Vec::new();
		let identifier = &self.identifier.to_string();
		if context.string.len() > 0 {
			context_vec.push(context.string);
			context_vec.push(identifier);
		} else if identifier != "_" {
			context_vec.push(identifier);
		}

		if context.is_static {
			match self.prefix {
				Prefix::Instance => {
					semantics.activate_element(&context, self.blocks.len());

					for rule in &self.rules {
						let context = Context {
							block: self,
							string: &context_vec.join("-"),
							is_static: context.is_static && self.prefix == Prefix::Instance,
							path: context.path.to_vec(),
							index: context.index,
						};
						rule.analyze(semantics, &context);
					}
				}
				Prefix::Class => {}
				Prefix::Action => {}
				Prefix::Listener => {}
			}
		}

		let mut i = 0;
		for block in &self.blocks {
			let mut path = context.path.to_vec();
			path.push(context.index);

			let context = Context {
				block: self,
				string: &context_vec.join("-"),
				is_static: context.is_static && self.prefix == Prefix::Instance,
				path,
				index: i,
			};
			i += 1;
			block.analyze(semantics, &context);
		}
	}
}

impl Analyze for Rule {
	fn analyze(&self, semantics: &mut Semantics, context: &Context) {
		let element = &mut semantics.get_element(&context);
		let property = &self.property.to_string()[..];
		let value = self.value.to_token_stream().to_string();
		let value = value[1..value.len() - 1].to_string();

		match context.block.prefix {
			Prefix::Instance => match property {
				"title" if context.is_root() => match &semantics.title {
					Some(_title) => semantics.error("title property cannot be set more than once"),
					None => semantics.title = Some(value),
				},

				"text" => {
					element.text = value.clone();
				}
				"link" => {
					// element.link = Some(expr_to_str(value));
				}
				"tooltip" => {
					// element.link = Some(expr_to_str(value));
				}
				_ => {}
			},
			Prefix::Class => {}
			Prefix::Action => {}
			Prefix::Listener => {}
		}
	}
}

use {
	data::{
		ast::{Block, Document, Prefix, Rule},
		dom::Event,
		Semantics,
	},
	misc::Context,
	syn::export::ToTokens,
};

pub fn semantic_analysis(document: &Document, semantics: &mut Semantics) {
	document.analyze(
		semantics,
		&Context {
			block: &document.root,
			index: 0,
			is_static: true,
			path: Vec::new(),
			string: "",
		},
	)
}

trait Analyze {
	fn analyze(&self, semantics: &mut Semantics, context: &Context);
}

impl Analyze for Document {
	fn analyze(&self, semantics: &mut Semantics, context: &Context) {
		self.root.analyze(semantics, &context);

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

		let mut child_context = Context {
			block: self,
			string: &context_vec.join("-"),
			is_static: context.is_static && self.prefix == Prefix::Instance,
			path: context.path.to_vec(),
			index: context.index,
		};

		if context.is_static {
			match self.prefix {
				Prefix::Instance => {
					semantics.activate_element(&context, self.blocks.len());

					for rule in &self.rules {
						rule.analyze(semantics, &child_context);
					}
				}
				Prefix::Class => {}
				Prefix::Action => {}
				Prefix::Listener => {
					let element = semantics.get_element(&context);
					element.listeners.push(Event::Click);
				}
			}
		}

		for (i, block) in self.blocks.iter().enumerate() {
			let mut path = context.path.to_vec();
			path.push(context.index);
			child_context.path = path;
			child_context.index = i;
			block.analyze(semantics, &child_context);
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
				"text" => element.text = value.clone(),
				"link" => element.link = Some(value.clone()),
				"tooltip" => element.tooltip = Some(value.clone()),
				_ => {}
			},
			Prefix::Class => {}
			Prefix::Action => {}
			Prefix::Listener => {}
		}
	}
}

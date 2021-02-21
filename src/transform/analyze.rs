use {
	data::{
		ast::{Block, Document, Prefix, Rule},
		Semantics,
	},
	misc::Context,
	quote::ToTokens,
};

impl Document {
	pub fn analyze(&self, bindgen: bool) -> Semantics {
		let mut semantics = Semantics::new(bindgen);
		self.root.analyze(
			&mut semantics,
			&Context {
				block: &self.root,
				path: Vec::new(),
				root: 0,
			},
			0,
		);

		semantics
	}
}

trait Analyze {
	fn analyze(&self, semantics: &mut Semantics, context: &Context, parent_id: usize);
}

impl Analyze for Block {
	fn analyze(&self, semantics: &mut Semantics, context: &Context, parent_id: usize) {
		let identifier = self.identifier.to_string();
		let group_id = match context.block.prefix {
			Prefix::Page => semantics.page_group(),
			Prefix::Instance => semantics.instance_group(identifier, parent_id),
			Prefix::Class => semantics.class_group(identifier, parent_id),
			Prefix::Action => 0,
			Prefix::Listener => parent_id,
		};

		for rule in &self.rules {
			let context = Context {
				block: context.block,
				path: context.path.clone(),
				root: context.root,
			};
			rule.analyze(semantics, &context, group_id);
		}

		for block in &self.blocks {
			let context = Context {
				block,
				path: context.path.clone(),
				root: context.root,
			};
			block.analyze(semantics, &context, group_id);
		}
	}
}

impl Analyze for Rule {
	fn analyze(&self, semantics: &mut Semantics, context: &Context, parent_id: usize) {
		let rules = &mut semantics.groups[parent_id].rules;
		let property = &*self.property.to_string();
		let value = self.value.to_token_stream().to_string();
		let value = value[1..value.len() - 1].to_string();

		match property {
			"title" if context.is_root() => semantics.title = Some(value),
			"route" if context.is_root() => rules.route = Some(value),
			"background_color" => rules.background_color = Some(value),
			"color" => rules.color = Some(value),
			"link" => rules.link = Some(value),
			"text" => rules.text = Some(value),
			"tooltip" => rules.tooltip = Some(value),
			_ => {}
		}
	}
}

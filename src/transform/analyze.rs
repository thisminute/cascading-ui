use {
	data::{
		ast::{Block, Document, Prefix, Property},
		CssProperty, CwlProperty, PageProperty, Semantics,
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
			None,
		);

		semantics
	}
}

trait Analyze {
	fn analyze(&self, semantics: &mut Semantics, context: &Context, group_id: Option<usize>);
}

impl Analyze for Block {
	fn analyze(&self, semantics: &mut Semantics, context: &Context, parent_id: Option<usize>) {
		let identifier = self.identifier.to_string();

		eprintln!("analyzing block with identifier {}", identifier);

		let group_id = if let Some(parent_id) = parent_id {
			match context.block.prefix {
				Prefix::Page => semantics.page_group(),
				Prefix::Instance => semantics.instance_group(identifier, parent_id),
				Prefix::Class => semantics.class_group(identifier, parent_id),
				Prefix::Action => 0,
				Prefix::Listener => parent_id,
			}
		} else {
			semantics.page_group()
		};

		for property in &self.properties {
			let context = Context {
				block: context.block,
				path: context.path.clone(),
				root: context.root,
			};
			property.analyze(semantics, &context, Some(group_id));
		}

		for block in &self.blocks {
			let context = Context {
				block: &block,
				path: context.path.clone(),
				root: context.root,
			};
			block.analyze(semantics, &context, Some(group_id));
		}
	}
}

impl Analyze for Property {
	fn analyze(&self, semantics: &mut Semantics, context: &Context, group_id: Option<usize>) {
		let properties = &mut semantics.groups[group_id.unwrap()].properties;
		let property = &*self.property.to_string();
		let value = self.value.to_token_stream().to_string();
		let value = value[1..value.len() - 1].to_string();
		eprintln!("Block: {} Rule: {}:{}", group_id.unwrap(), property, value);

		match property {
			// page properties
			"title" if context.is_root() => properties.page.insert(PageProperty::Title, value),
			"route" if context.is_root() => properties.page.insert(PageProperty::Route, value),

			// css properties
			"background_color" => properties.css.insert(CssProperty::BackgroundColor, value),

			"color" => properties.css.insert(CssProperty::Color, value),

			"link" => properties.cwl.insert(CwlProperty::Link, value),
			"text" => properties.cwl.insert(CwlProperty::Text, value),
			"tooltip" => properties.cwl.insert(CwlProperty::Tooltip, value),
			"image" => properties.cwl.insert(CwlProperty::Image, value),
			_ => None,
		};
	}
}

use {
	data::{
		ast::{Block, Document, Prefix, Property},
		semantics::{
			properties::{CssProperty, CwlProperty, PageProperty},
			Group, Page, Semantics,
		},
	},
	quote::ToTokens,
};

impl Document {
	pub fn analyze(self) -> Semantics {
		let mut semantics = Semantics::default();
		semantics.styles.insert(
			"body".into(),
			[(CssProperty::Margin, "0".into())]
				.iter()
				.cloned()
				.collect(),
		);
		semantics.styles.insert(
			"a".into(),
			[(CssProperty::Display, "block".into())]
				.iter()
				.cloned()
				.collect(),
		);
		eprintln!("...Creating groups...");
		semantics.create_group_from_block(self.root, None, None, true);
		semantics
	}
}

impl Semantics {
	fn create_group_from_block(
		&mut self,
		block: Block,
		mut page_id: Option<usize>,
		parent_id: Option<usize>,
		mut r#static: bool,
	) -> usize {
		eprintln!(
			"Analyzing {:?} block with identifier {}",
			block.prefix,
			block.identifier.to_string()
		);

		let identifier = block.identifier.to_string();
		let group_id = self.groups.len();
		let group = if let Some(parent_id) = parent_id {
			let parent = &mut self.groups[parent_id];
			match block.prefix {
				Prefix::Element => {
					parent.elements.push(group_id);
				}
				Prefix::Class => {
					parent
						.classes
						.entry(identifier.clone())
						.or_default()
						.push(group_id);
				}
				Prefix::Listener => {
					r#static = false;
					parent.listeners.push(group_id);
				}
			}
			Group::new(Some(identifier.clone()), r#static)
		} else {
			let group = Group::new(None, r#static);
			page_id = Some(self.pages.len());
			self.pages.push(Page {
				title: String::from(""),
				route: String::from("/"),
				root_id: group_id,
			});
			group
		};
		self.groups.push(group);

		for property in block.properties {
			self.apply_static_property(property, group_id);
		}

		for block in block.classes {
			self.create_group_from_block(block, page_id, Some(group_id), r#static);
		}
		for block in block.listeners {
			self.create_group_from_block(block, page_id, Some(group_id), r#static);
		}
		for block in block.elements {
			self.create_group_from_block(block, page_id, Some(group_id), r#static);
		}

		group_id
	}

	fn apply_static_property(&mut self, property: Property, group_id: usize) {
		let group = &mut self.groups[group_id];
		let properties = &mut group.properties;
		let (property, value) = (
			property.property.to_string(),
			property.value.to_token_stream().to_string(),
		);
		let value = value[1..value.len() - 1].to_string();
		eprintln!(
			" Applying property {}:{} to group {}",
			property, value, group_id
		);

		if let Some(value) = match &*property {
			// page properties
			"title" => properties.page.insert(PageProperty::Title, value),
			"route" => properties.page.insert(PageProperty::Route, value),

			// css properties
			"background_color" => properties.css.insert(CssProperty::BackgroundColor, value),
			"color" => properties.css.insert(CssProperty::Color, value),
			"position" => properties.css.insert(CssProperty::Position, value),
			"height" => properties.css.insert(CssProperty::Height, value),
			"width" => properties.css.insert(CssProperty::Width, value),

			"link" => properties.cwl.insert(CwlProperty::Link, value),
			"text" => properties.cwl.insert(CwlProperty::Text, value),
			"tooltip" => properties.cwl.insert(CwlProperty::Tooltip, value),
			"image" => properties.cwl.insert(CwlProperty::Image, value),
			_ => {
				eprintln!("Unrecognized property {}", property);
				panic!("Unrecognized property");
			}
		} {
			eprintln!("Overwrote old value of {}", value)
		}
	}
}

use data::{
	ast::{Block, Document, Prefix, Value as AstValue},
	semantics::{
		properties::{is_css_property, CuiProperty},
		Group, Page, Semantics, StaticValue, Value,
	},
};

impl Document {
	pub fn analyze(self) -> Semantics {
		let mut semantics = Semantics::default();
		semantics.styles.insert(
			"body".to_string(),
			[(
				"margin".to_string(),
				Value::Static(StaticValue::String("0".to_string())),
			)]
			.iter()
			.cloned()
			.collect(),
		);
		semantics.styles.insert(
			"a".to_string(),
			[(
				"display".to_string(),
				Value::Static(StaticValue::String("block".to_string())),
			)]
			.iter()
			.cloned()
			.collect(),
		);
		log::debug!("...Creating groups...");
		semantics.create_group_from_block(self.root, None, None, None);
		semantics
	}
}

impl Semantics {
	fn create_group_from_block(
		&mut self,
		block: Block,
		mut page_id: Option<usize>,
		parent_id: Option<usize>,
		mut listener_scope: Option<usize>,
	) {
		log::debug!(
			"Analyzing {:?} block with identifier {}",
			block.prefix,
			block.identifier.to_string()
		);

		let variables = block
			.variables
			.iter()
			.map(|(identifier, value)| {
				let value = self.create_semantic_value(value);
				(identifier.clone(), value)
			})
			.collect();

		let group_id = self.groups.len();
		let group = if let Some(parent_id) = parent_id {
			let identifier = block.identifier.to_string();
			let parent = &mut self.groups[parent_id];
			let current_scope = listener_scope;
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
					listener_scope = Some(group_id);
					parent.listeners.push(group_id);
				}
			}
			Group::new(Some(identifier.clone()), current_scope, variables)
		} else {
			page_id = Some(self.pages.len());
			self.pages.push(Page {
				title: String::from(""),
				route: String::from("/"),
				root_id: group_id,
			});
			Group::new(None, None, variables)
		};
		self.groups.push(group);

		for property in block.properties {
			let (property, value) = (property.property.to_string(), property.value);
			log::debug!(
				" Applying property {}:{:?} to group {}",
				property,
				value,
				group_id
			);
			let value = self.create_semantic_value(&value);

			let group = &mut self.groups[group_id];
			let properties = &mut group.properties;
			if is_css_property(&property) {
				properties.css.insert(property, value);
			} else {
				properties.cui.insert(CuiProperty(property), value);
			}
			// page properties
			// "title" => properties.page.insert(PageProperty::Title, value),
		}

		for block in block.listeners {
			self.create_group_from_block(block, page_id, Some(group_id), listener_scope);
		}
		for block in block.classes {
			self.create_group_from_block(block, page_id, Some(group_id), listener_scope);
		}
		for block in block.elements {
			self.create_group_from_block(block, page_id, Some(group_id), listener_scope);
		}
	}

	fn create_semantic_value(&self, value: &AstValue) -> Value {
		match value {
			AstValue::Variable(identifier) => Value::Variable(identifier.0.to_string(), None, None),
			AstValue::Number(value) => Value::Static(StaticValue::Number(*value)),
			AstValue::String(value) => Value::Static(StaticValue::String(value.clone())),
		}
	}
}

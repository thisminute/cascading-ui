pub mod cascade;

use {
	data::{
		ast::{Block, Document, Prefix, Property},
		semantics::{
			properties::{CssProperty, CwlProperty, PageProperty},
			Group, Page, Semantics,
		},
	},
	quote::ToTokens,
	std::collections::HashMap,
};

impl Document {
	pub fn analyze(self) -> Semantics {
		let mut semantics = Semantics::default();
		eprintln!("Creating groups...");
		semantics.create_group_from_block(self.root, None);
		eprintln!("Applying classes...");
		semantics.apply_classes(0);
		semantics
	}
}

impl Semantics {
	fn create_group_from_block(&mut self, block: Block, parent_id: Option<usize>) {
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
				Prefix::Action => {}
				Prefix::Listener => {
					parent.listeners.push((String::default(), group_id));
				}
			}
			Group::new(Some(parent_id), Some(identifier))
		} else {
			let group = Group::new(None, None);
			self.pages.push(Page {
				title: String::from(""),
				route: String::from("/"),
				styles: HashMap::new(),
				root_id: group_id,
			});
			group
		};

		self.groups.push(group);

		for property in block.properties {
			self.apply_property(property, group_id);
		}

		for block in block.classes {
			self.create_group_from_block(block, Some(group_id));
		}

		for block in block.elements {
			self.create_group_from_block(block, Some(group_id));
		}

		for block in block.listeners {
			self.create_group_from_block(block, Some(group_id));
		}
	}

	fn apply_property(&mut self, property: Property, group_id: usize) {
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
			"title" if group.parent_id.is_none() => {
				properties.page.insert(PageProperty::Title, value)
			}
			"route" if group.parent_id.is_none() => {
				properties.page.insert(PageProperty::Route, value)
			}

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

	fn apply_classes(&mut self, group_id: usize) {
		eprintln!("Applying properties from classes to group {}", group_id);
		if let Some(name) = &self.groups[group_id].name.clone() {
			let mut ancestor = &self.groups[group_id];
			let mut queue = Vec::new();
			while let Some(parent_id) = ancestor.parent_id {
				ancestor = &mut self.groups[parent_id];
				for &subgroup_id in ancestor.classes.get(name).unwrap_or(&Vec::new()) {
					queue.push((subgroup_id, group_id));
				}
			}
			for (subgroup_id, member_id) in queue {
				eprintln!("Adding member {} to class {}", member_id, subgroup_id);
				self.groups[subgroup_id].members.push(member_id);
				self.groups[member_id].member_of.push(subgroup_id);
			}
		}

		for &class_id in &self.groups[group_id].member_of.clone() {
			self.cascade(class_id, group_id);
		}

		for &element_id in &self.groups[group_id].elements.clone() {
			self.apply_classes(element_id);
		}
	}
}

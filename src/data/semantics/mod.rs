pub mod event;
pub mod properties;

use {
	self::properties::{CssProperties, PageProperty, Properties},
	std::{collections::HashMap, error::Error, fmt},
};

#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "There is an error: {}", self.0)
	}
}
impl Error for MyError {}

pub struct CssRule {
	pub selector: String,
	pub properties: CssProperties,
}

pub struct Group {
	pub members: Vec<usize>,
	pub properties: Properties,

	pub classes: Vec<String>,

	pub parent_id: Option<usize>,
	pub children: Vec<usize>,
	pub subgroups: HashMap<String, usize>,
}

pub struct Semantics {
	pub only_header_wasm: bool,
	pub bindgen: bool,

	pub errors: Vec<&'static str>,
	pub warnings: Vec<&'static str>,

	pub pages: Vec<usize>,
	pub groups: Vec<Group>,
}
impl Semantics {
	pub fn new(bindgen: bool) -> Self {
		Self {
			only_header_wasm: false,
			bindgen,

			errors: Vec::new(),
			warnings: Vec::new(),

			pages: Vec::new(),
			groups: Vec::new(),
		}
	}

	pub fn _error(&mut self, message: &'static str) {
		self.errors.push(message);
	}

	pub fn _warning(&mut self, message: &'static str) {
		self.warnings.push(message);
	}

	pub fn page_group(&mut self) -> usize {
		let id = self.groups.len();
		let mut page = Group {
			parent_id: None,
			members: Vec::new(),
			properties: Properties::default(),

			classes: Vec::new(),
			children: Vec::new(),
			subgroups: HashMap::new(),
		};
		eprintln!("Generating page group {}", id);
		page.members.push(id);
		if id == 0 {
			page.properties
				.page
				.insert(PageProperty::Route, String::from("/"));
		}
		self.pages.push(id);

		self.groups.push(page);
		id
	}

	pub fn instance_group(&mut self, identifier: String, parent_id: usize) -> usize {
		let id = self.groups.len();
		let mut instance = Group {
			parent_id: Some(parent_id),
			members: vec![id],
			properties: Properties::default(),

			classes: Vec::new(),
			children: Vec::new(),
			subgroups: HashMap::new(),
		};

		// add a reference to this group to every ancestor group that has a class definition for this group's identifier
		let mut queue = Vec::new();
		let mut ancestor = &mut instance;
		while let Some(parent_id) = ancestor.parent_id {
			ancestor = &mut self.groups[parent_id];
			if let Some(subgroup) = ancestor.subgroups.get(&identifier) {
				queue.push((subgroup.clone(), id));
			}
		}

		for (group_id, member_id) in queue {
			eprintln!("Adding member {} to group {}", member_id, group_id);
			self.groups[group_id].members.push(member_id);
		}

		self.groups[parent_id].children.push(id);
		self.groups.push(instance);
		id
	}

	pub fn class_group(&mut self, identifier: String, parent_id: usize) -> usize {
		let id = self.groups.len();
		let class = Group {
			parent_id: Some(parent_id),
			members: Vec::new(),
			properties: Properties::default(),

			classes: Vec::new(),
			children: Vec::new(),
			subgroups: HashMap::new(),
		};
		self.groups[parent_id].subgroups.insert(identifier, id);
		self.groups.push(class);
		id
	}

	// pub fn activate_element(&mut self, context: &Context, size: usize) -> &Element {
	// 	let mut current = &mut self.dom[context.root];
	// 	for i in context.path {
	// 		current.active = true;
	// 		current = &mut current.children[&i];
	// 	}

	// 	current.active = true;
	// 	current.children.reserve_exact(size);
	// 	for _ in 0..size {
	// 		current.children.push(Element::new());
	// 	}
	// 	current
	// }

	// pub fn get_element(&mut self, context: &Context) -> &Element {
	// 	let mut current = &self.dom[context.root];
	// 	for i in context.path {
	// 		current = &current.children[i];
	// 	}
	// 	current
	// }

	// pub fn get_group(&mut self, group_id: usize) -> &Group {
	// 	&self.groups[group_id]
	// }

	// pub fn _attach(&mut self, _element: &Element, _group: usize) {}
	// pub fn _queue_attach(&mut self, _context: &Context) {}
}

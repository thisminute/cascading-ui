pub mod event;
pub mod rules;

use {
	self::rules::Rules,
	// misc::{id_gen, Context},
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

pub struct Group {
	// id: usize,
	pub parent_id: Option<usize>,
	// class: Option<String>,
	pub children: Vec<usize>,
	pub subgroups: HashMap<String, Vec<usize>>,

	pub rules: Rules,
}

// pub struct Class<'a> {
// 	pub text: &'a str,
// 	pub styles: Vec<&'a str>,
// }
// impl Default for Class<'_> {
// 	fn default() -> Self {
// 		Self {
// 			text: "",
// 			styles: Vec::new(),
// 		}
// 	}
// }

pub struct Semantics {
	pub only_header_wasm: bool,
	pub bindgen: bool,

	pub errors: Vec<&'static str>,
	pub warnings: Vec<&'static str>,

	pub title: Option<String>,
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

			title: None,
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
			// id,
			parent_id: None,
			rules: Rules::new(),
			// class: None,
			children: Vec::new(),
			subgroups: HashMap::new(),
		};
		if id == 0 {
			page.rules.route = Some("/".into())
		}
		self.groups.push(page);
		self.pages.push(id);
		id
	}

	pub fn instance_group(&mut self, identifier: String, parent_id: usize) -> usize {
		let id = self.groups.len();
		let mut instance = Group {
			// id,
			parent_id: Some(parent_id),
			rules: Rules::new(),
			// class: None,
			children: Vec::new(),
			subgroups: HashMap::new(),
		};

		// add a reference to this group to every ancestor group that has a class definition for this group's identifier
		let mut ancestor = &mut instance;
		while let Some(parent) = ancestor.parent_id {
			ancestor = &mut self.groups[parent];
			if let Some(subgroup) = ancestor.subgroups.get_mut(&identifier) {
				subgroup.push(parent);
			}
		}
		self.groups[parent_id].children.push(id);
		self.groups.push(instance);
		id
	}

	pub fn class_group(&mut self, identifier: String, parent_id: usize) -> usize {
		let id = self.groups.len();
		self.groups[parent_id]
			.subgroups
			.insert(identifier, Vec::new())
			.expect("inserting group");
		self.groups.push(Group {
			// id,
			parent_id: Some(parent_id),
			rules: Rules::new(),
			// class: Some(identifier),
			children: Vec::new(),
			subgroups: HashMap::new(),
		});
		id
	}

	// pub fn activate_element(&mut self, context: &Context, size: usize) -> &Element {
	// 	let mut current = &mut self.dom[context.root];
	// 	for i in &context.path {
	// 		current.active = true;
	// 		current = &mut current.children[*i];
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
	// 	for i in &context.path {
	// 		current = &current.children[*i];
	// 	}
	// 	current
	// }

	// pub fn get_group(&mut self, group_id: usize) -> &Group {
	// 	&self.groups[group_id]
	// }

	// pub fn _attach(&mut self, _element: &Element, _group: usize) {}
	// pub fn _queue_attach(&mut self, _context: &Context) {}
}

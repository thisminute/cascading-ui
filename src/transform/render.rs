use crate::misc::id_gen::IdCategory;

use {
	data::{dom::Page, semantics::Group, Dom, Element, Semantics},
	misc::id_gen::id_gen,
};

type Groups = Vec<Group>;

trait Render {
	fn render_1(&mut self, group_id: usize);
	fn render_2(&mut self, group_id: usize) -> Element;
}

impl Semantics {
	pub fn render(&mut self) -> Dom {
		let mut dom = Dom::new();
		for &page in &self.pages {
			self.groups.render_1(page);
			let root = self.groups.render_2(page);
			let page = &mut self.groups[page];
			dom.html_roots.insert(
				page.properties
					.route
					.clone()
					.expect("a route must be set for all pages (index defaults to \"/\")"),
				Page {
					title: page
						.properties
						.title
						.clone()
						.expect("a title must be set for the home page"),
					root,
				},
			);
		}
		dom
	}
}

impl Render for Groups {
	fn render_1(&mut self, group_id: usize) {
		if self[group_id].members.len() == 0 {
			panic!("get rid of groups with no members")
		}
		if self[group_id].members.len() == 1 {
			// TODO: currently just overwrites styles, should merge them instead
			let member_id = self[group_id].members[0];
			self[member_id].styles = self[group_id].properties.css.clone();
		} else {
			let mut queue = Vec::new();
			let class_id = id_gen(IdCategory::Class);
			for &member in &self[group_id].members {
				queue.push((member, class_id.clone()));
			}
			for (group_id, class_id) in queue {
				self[group_id].classes.push(class_id);
			}
		}

		let mut queue = Vec::new();
		for &child_id in &self[group_id].children {
			queue.push(child_id);
		}
		for (_, &group_id) in &self[group_id].subgroups {
			queue.push(group_id);
		}
		for item in queue {
			self.render_1(item);
		}
	}

	fn render_2(&mut self, group_id: usize) -> Element {
		if self[group_id].members.len() == 0 {
			panic!("get rid of groups with no members")
		}

		let mut element = {
			let group = &self[group_id];
			Element {
				id: if group.subgroups.len() > 0 {
					Some(id_gen(IdCategory::Id))
				} else {
					None
				},
				link: group.properties.link.clone(),
				text: group.properties.text.clone().unwrap_or_default(),
				classes: group.classes.clone(),
				style: group.styles.clone(),
				children: Vec::new(),
				listeners: Vec::new(),
			}
		};

		let mut queue = Vec::new();
		for &child in &self[group_id].children {
			queue.push(child);
		}
		for child in queue {
			element.children.push(self.render_2(child));
		}
		element
	}
}

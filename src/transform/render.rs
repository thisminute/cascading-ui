use {
	crate::misc::id_gen::IdCategory,
	data::{dom::Page, semantics::Group, CssRule, Dom, Element, Semantics},
	misc::id_gen::id_gen,
	transform::write::css::Css,
};

type Groups = Vec<Group>;

trait Render {
	fn render_1(&mut self, group_id: usize, styles: &mut Vec<CssRule>);
	fn render_2(&mut self, group_id: usize) -> Element;
}

impl Semantics {
	pub fn render(&mut self) -> Dom {
		let mut dom = Dom::new();
		for &page in &self.pages {
			eprintln!("rendering page {}", page);
			let mut styles = Vec::new();
			self.groups.render_1(page, &mut styles);
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
					styles,
					root,
				},
			);
		}
		dom
	}
}

impl Render for Groups {
	fn render_1(&mut self, group_id: usize, styles: &mut Vec<CssRule>) {
		eprintln!("render 1 for group {}", group_id);
		if self[group_id].members.len() == 0 {
			panic!("get rid of groups with no members")
		}
		if self[group_id].members.len() == 1 {
			// TODO: currently just overwrites styles, should merge them instead
			let member_id = self[group_id].members[0];
			eprintln!("applying styles to group {}", member_id);
			self[member_id].styles = self[group_id].properties.css.clone();
		} else {
			let mut queue = Vec::new();
			let class = id_gen(IdCategory::Class);
			for &member_id in &self[group_id].members {
				queue.push((member_id, class.clone()));
			}
			for (group_id, class) in queue {
				self[group_id].classes.push(class);
			}
			styles.push(CssRule {
				selector: format!(".{}", class),
				properties: self[group_id].properties.css.clone(),
			});
		}

		let mut queue = Vec::new();
		for &child_id in &self[group_id].children {
			queue.push(child_id);
		}
		for (_, &group_id) in &self[group_id].subgroups {
			queue.push(group_id);
		}
		for item in queue {
			self.render_1(item, styles);
		}
	}

	fn render_2(&mut self, group_id: usize) -> Element {
		eprintln!("render 2 on group {}", group_id);
		if self[group_id].members.len() == 0 {
			panic!("get rid of groups with no members")
		}

		let mut element = {
			let group = &self[group_id];
			eprintln!("style {}", group.styles.css());
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
			eprintln!("adding a child");
			queue.push(child);
		}
		for child in queue {
			element.children.push(self.render_2(child));
		}
		element
	}
}

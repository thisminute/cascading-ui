use {
	crate::misc::id_gen::IdCategory,
	data::{
		dom::Page,
		semantics::{
			properties::{CssProperty, CwlProperty, PageProperty},
			Group,
		},
		CssRule, Dom, Element, Semantics,
	},
	misc::id_gen::id_gen,
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
			let mut styles = vec![
				CssRule {
					selector: String::from("body"),
					properties: [(CssProperty::Margin, 0.to_string())]
						.iter()
						.cloned()
						.collect(),
				},
				CssRule {
					selector: String::from("a"),
					properties: [(CssProperty::Display, "block".to_string())]
						.iter()
						.cloned()
						.collect(),
				},
			];
			self.groups.render_1(page, &mut styles);
			let root = self.groups.render_2(page);
			let page = &mut self.groups[page];
			dom.html_roots.insert(
				page.properties
					.page
					.get(&PageProperty::Route)
					.expect("a route must be set for all pages (index defaults to \"/\")")
					.to_string(),
				Page {
					title: page
						.properties
						.page
						.get(&PageProperty::Title)
						.expect("a title must be set for the home page")
						.to_string(),
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
		let incoming_properties = self[group_id].properties.clone();
		if self[group_id].members.len() == 1 {
			let &member_id = self[group_id].members.first().unwrap();
			self[member_id]
				.properties
				.cascade(&incoming_properties, true);
		} else {
			let mut queue = Vec::new();
			let class = id_gen(IdCategory::Class);
			for &member_id in &self[group_id].members {
				queue.push((member_id, class.clone()));
			}
			for (member_id, class) in queue {
				self[member_id].classes.push(class);
				self[member_id]
					.properties
					.cascade(&incoming_properties, false);
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
			eprintln!(
				"text {}",
				group
					.properties
					.cwl
					.get(&CwlProperty::Text)
					.unwrap_or(&format!(""))
					.clone()
			);
			Element {
				id: if group.subgroups.len() > 0 {
					Some(id_gen(IdCategory::Id))
				} else {
					None
				},
				link: match group.properties.cwl.get(&CwlProperty::Link) {
					Some(url) => Some(url.clone()),
					None => None,
				},
				text: group
					.properties
					.cwl
					.get(&CwlProperty::Text)
					.unwrap_or(&format!(""))
					.clone(),
				classes: group.classes.clone(),
				style: group.properties.css.clone(),
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

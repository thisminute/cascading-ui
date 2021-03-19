use {
	super::cascade::Cascade,
	data::{
		dom::{CssRule, Dom, Element, Listener, Page},
		semantics::{
			properties::{CssProperty, CwlProperty, PageProperty},
			Group, Semantics,
		},
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
		eprintln!("render_1 for group {}", group_id);
		if self[group_id].properties.css.len() > 0 {
			match self[group_id].members.len() {
				0 => {}
				1 => {
					let &member_id = self[group_id].members.first().unwrap();
					self.cascade_css(group_id, member_id);
				}
				_ => {
					let class = id_gen();
					self[group_id].id = Some(class.clone());
					styles.push(CssRule {
						selector: format!(".{}", class),
						properties: self[group_id].properties.css.clone(),
					});
				}
			}
		}

		for &child_id in &self[group_id].elements.clone() {
			self.render_1(child_id, styles);
		}
		for (_, group_ids) in &self[group_id].classes.clone() {
			for &group_id in group_ids {
				self.render_1(group_id, styles);
			}
		}
	}

	fn render_2(&mut self, group_id: usize) -> Element {
		eprintln!("render_2 on group {}", group_id);
		let group = &self[group_id];
		let mut classes = group
			.member_of
			.iter()
			.filter(|&&member_id| self[member_id].id.is_some())
			.map(|&member_id| self[member_id].id.clone().unwrap())
			.collect::<Vec<String>>();
		let listeners = self[group_id]
			.listeners
			.iter()
			.map(|&listener_id| {
				let id = id_gen();
				classes.push(id.clone());
				Listener {
					event: self[listener_id]
						.name
						.clone()
						.expect("all listeners should have an associated event type"),
					id,
					properties: self[listener_id].properties.clone(),
				}
			})
			.collect();
		Element {
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
			classes,
			style: group.properties.css.clone(),
			children: group
				.elements
				.clone()
				.iter()
				.map(|&child| self.render_2(child))
				.collect(),
			listeners,
		}
	}
}

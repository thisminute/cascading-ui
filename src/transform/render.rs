use {
	data::semantics::{
		properties::{CssProperty, CssRules, PageProperty},
		Semantics,
	},
	misc::id_gen::id_gen,
};

impl Semantics {
	pub fn render(&mut self) {
		for i in 0..self.pages.len() {
			let page = &mut self.pages[i];
			eprintln!("rendering page {}", page.root_id);
			let page_root = &mut self.groups[page.root_id];
			if let Some(route) = page_root.properties.page.get(&PageProperty::Route) {
				page.route = route.into();
			}
			page.title = page_root
				.properties
				.page
				.get(&PageProperty::Title)
				.expect("a title must be set for the home page")
				.clone();
			let mut styles = [
				(
					"body".into(),
					[(CssProperty::Margin, "0".into())]
						.iter()
						.cloned()
						.collect(),
				),
				(
					"a".into(),
					[(CssProperty::Display, "block".into())]
						.iter()
						.cloned()
						.collect(),
				),
			]
			.iter()
			.cloned()
			.collect();
			self.render_css(i, &mut styles);
			self.pages[i].styles = styles;
		}
	}
}

impl Semantics {
	fn render_css(&mut self, group_id: usize, styles: &mut CssRules) {
		eprintln!("render_css for group {}", group_id);
		if !self.groups[group_id].properties.css.is_empty() {
			for member_id in self.groups[group_id].members.clone() {
				if self.groups[group_id].members.len() == 1 {
					let css = self.groups[group_id].properties.css.clone();
					self.groups[member_id].cascade_css(css);
				} else {
					let class = id_gen();
					self.groups[group_id].selector = Some(class.clone());
					for member_id in self.groups[group_id].members.clone() {
						self.groups[member_id].class_names.push(class.clone());
					}
					styles.insert(
						format!(".{}", class),
						self.groups[group_id].properties.css.clone(),
					);
				}
			}
		}

		for child_id in self.groups[group_id].elements.clone() {
			self.render_css(child_id, styles);
		}
		for (_, group_ids) in self.groups[group_id].classes.clone() {
			for group_id in group_ids {
				self.render_css(group_id, styles);
			}
		}
		for (class, _) in self.groups[group_id].listeners.clone() {
			self.groups[group_id].class_names.push(class);
		}
	}
}
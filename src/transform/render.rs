use data::{dom::Page, semantics::Group, Dom, Element, Semantics};

impl Semantics {
	pub fn render(&self) -> Dom {
		let mut dom = Dom::new();
		for &page in &self.pages {
			let page = &self.groups[page];
			let root = page.render(self);
			dom.html_roots.insert(
				page.rules
					.route
					.clone()
					.expect("a route must be set for all pages except for the index"),
				Page {
					title: self
						.title
						.clone()
						.expect("the home page must have a title set"),
					root,
				},
			);
		}
		dom
	}
}

impl Group {
	fn render(&self, semantics: &Semantics) -> Element {
		Element {
			active: true,
			children: self
				.children
				.iter()
				.map(|&child| semantics.groups[child].render(semantics))
				.collect(),
			listeners: Vec::new(),
			rules: self.rules.clone(),
		}
	}
}

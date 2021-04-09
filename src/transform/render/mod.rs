mod cascade;
mod element;

use data::semantics::{properties::PageProperty, Semantics};

impl Semantics {
	pub fn render(&mut self) {
		eprintln!("...Rendering...");
		for i in 0..self.pages.len() {
			let page_group_id = self.pages[i].root_id;
			eprintln!("Rendering page {}", page_group_id);
			let page = &mut self.groups[page_group_id];
			if let Some(route) = page.properties.page.get(&PageProperty::Route) {
				self.pages[i].route = route.into();
			}
			self.pages[i].title = page
				.properties
				.page
				.get(&PageProperty::Title)
				.expect("a title must be set for the home page")
				.into();
			self.render_element(page_group_id, &mut Vec::new());
		}
	}
}

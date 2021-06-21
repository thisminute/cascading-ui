mod cascade;
mod element;

use data::{
	ast::Value,
	semantics::{properties::PageProperty, Semantics},
};

impl Semantics {
	pub fn render(&mut self) {
		log::debug!("...Rendering...");
		for i in 0..self.pages.len() {
			let page_group_id = self.pages[i].root_id;
			log::debug!("Rendering page {}", page_group_id);
			let page = &mut self.groups[page_group_id];
			// TODO: routes based on directory structure
			if let Value::String(title) = &page
				.properties
				.page
				.get(&PageProperty::Title)
				.or(Some(&Value::String("".to_string())))
				.unwrap()
			{
				self.pages[i].title = title.clone();
				self.render_element(page_group_id, &mut Vec::new());
			} else {
				panic!("invalid title provided");
			}
		}
	}
}

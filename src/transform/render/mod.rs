mod cascade;
mod element;
mod value;

use data::semantics::{
	properties::{PageProperty, Property},
	Semantics, StaticValue, Value,
};

impl Semantics {
	pub fn render(&mut self) {
		log::debug!("...Rendering...");
		for i in 0..self.pages.len() {
			let page_group_id = self.pages[i].root_id;
			log::debug!("Rendering page {}", page_group_id);
			// TODO: routes based on directory structure
			let ancestors = &mut Vec::new();
			let default = &Value::Static(StaticValue::String("".into()));
			let title = self.groups[page_group_id]
				.properties
				.get(&Property::Page(PageProperty::Title))
				.unwrap_or(default)
				.clone();
			self.pages[i].title = title;
			self.render_element(page_group_id, ancestors);
		}
	}
}

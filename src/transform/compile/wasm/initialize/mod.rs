mod register;
mod render;

use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote, std::convert::TryInto};

impl Semantics {
	pub fn document(&self) -> TokenStream {
		let rules = self.static_elements(self.pages[0].root_id);
		quote! {
			CLASSES.with(|classes| {
				let window = web_sys::window().expect("getting window");
				let document = &window.document().expect("getting `window.document`");
				let head = &document.head().expect("getting `window.document.head`");
				let body = &document.body().expect("getting `window.document.body`");
				let mut classes = classes.borrow_mut();

				let element = body
					.children()
					.item(0)
					.expect("body should have a root element")
					.dyn_into::<HtmlElement>()
					.unwrap();
				#rules
			});
		}
	}

	fn static_elements(&self, element_id: usize) -> TokenStream {
		let elements = self.groups[element_id]
			.elements
			.iter()
			.enumerate()
			.map(|(i, &child_id)| {
				let i: u32 = i.try_into().unwrap();
				let rules = self.static_elements(child_id);
				quote! {
					{
						let element = element
							.children()
							.item(#i)
							.expect("should never try to index into an empty element")
							.dyn_into::<HtmlElement>()
							.unwrap();
						#rules
					}
				}
			})
			.collect::<TokenStream>();
		let classes = self.static_classes(element_id);
		let listeners = self.static_render_listeners(element_id);
		quote! {
			#elements
			#classes
			#listeners
		}
	}

	fn static_classes(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.classes
			.iter()
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				let selector = self.groups[class_id]
					.selector
					.clone()
					.expect("static and dynamic classes should have selectors");
				let rules = self.static_register_all(class_id);
				quote! {
					{
						let mut class = classes.entry(#selector).or_insert(Group::default());
						#rules
					}
				}
			})
			.collect()
	}
}

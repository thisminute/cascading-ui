use {
	data::semantics::{properties::CuiProperty, Semantics},
	proc_macro2::TokenStream,
	quote::quote,
};

impl Semantics {
	pub fn static_render_all(&self, group_id: usize) -> TokenStream {
		let elements = self.static_render_elements(group_id);
		let classes = self.static_render_classes(group_id);
		let listeners = self.static_render_listeners(group_id);
		let properties = self.static_render_properties(group_id);
		quote! {
			#elements
			#classes
			#listeners
			#properties
		}
	}

	fn static_render_elements(&self, group_id: usize) -> TokenStream {
		if self.groups[group_id].elements.is_empty() {
			return quote! {};
		}

		let elements = self.groups[group_id].elements.iter().map(|&element_id| {
			let rules = self.static_render_all(element_id);
			let tag = self.groups[element_id].tag;
			let class_names = &self.groups[element_id].class_names;
			quote! {
				element.append_child({
					let mut element = static_render_element(#tag, vec![#( #class_names ),*], &mut classes);
					#rules
					&element.into()
				}).unwrap();
			}
		});
		quote! {
			while let Some(child) = element.last_element_child() {
				element.remove_child(&child.dyn_into::<Node>().unwrap()).unwrap();
			}

			#( #elements )*
		}
	}

	fn static_render_classes(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.classes
			.iter()
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				let selector = self.groups[class_id]
					.selector
					.as_ref()
					.expect("dynamic classes should have selectors");
				let rules = self.static_render_all(class_id);
				let queue = self.static_register_all(class_id);
				quote! {
					let elements = document.get_elements_by_class_name(#selector);
					for i in 0..elements.length() {
						let mut element = elements
							.item(i)
							.unwrap()
							.dyn_into::<HtmlElement>()
							.unwrap();
						#rules
					}
					let mut class = classes.entry(#selector).or_insert(Group::default());
					#queue
				}
			})
			.collect::<TokenStream>()
	}

	pub fn static_render_listeners(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.listeners
			.iter()
			.map(|&listener_id| {
				let rules = self.static_render_all(listener_id);
				let event = match &**self.groups[listener_id]
					.name
					.as_ref()
					.expect("every listener should have an event id")
				{
					"blur" => quote! { set_onblur },
					"focus" => quote! { set_onfocus },
					"click" => quote! { set_onclick },
					"mouseover" => quote! { set_onmouseover },
					"mouseenter" => quote! { set_onmouseenter },
					"mouseleave" => quote! { set_onmouseleave },
					"mouseout" => quote! { set_onmouseout },
					_ => panic!("unknown event id"),
				};
				quote! {
					let closure = {
						let mut element = element.clone();
						Closure::wrap(Box::new(move |e: Event| {
							e.stop_propagation();
							let window = web_sys::window().unwrap();
							let document = window.document().unwrap();
							CLASSES.with(|classes| {
								let mut classes = classes.borrow_mut();
								#rules
							});
						}) as Box<dyn FnMut(Event)>)
					};
					element.#event(Some(closure.as_ref().unchecked_ref()));
					closure.forget();
				}
			})
			.collect()
	}

	fn static_render_properties(&self, group_id: usize) -> TokenStream {
		let properties = &self.groups[group_id].properties;
		let mut effects = Vec::new();
		if let Some(value) = properties.cui.get(&CuiProperty("text".to_string())) {
			effects.push(quote! { element.text(#value); });
		}
		if let Some(_value) = properties.cui.get(&CuiProperty("link".to_string())) {
			effects.push(quote! {});
		}

		for (property, value) in &properties.css {
			effects.push(quote! { element.css(#property, #value); });
		}

		quote! { #( #effects )* }
	}
}

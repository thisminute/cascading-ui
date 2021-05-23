use {
	data::semantics::{properties::CwlProperty, Semantics},
	proc_macro2::TokenStream,
	quote::quote,
	transform::compile::css::Css,
};

impl Semantics {
	pub fn apply_all(&self, group_id: usize) -> TokenStream {
		let classes = self.apply_classes(group_id);
		let listeners = self.apply_listeners(group_id);
		let elements = self.apply_elements(group_id);
		let properties = self.apply_properties(group_id);
		quote! {
			#classes
			#listeners
			#elements
			#properties
		}
	}

	pub fn apply_classes(&self, group_id: usize) -> TokenStream {
		let apply = self.groups[group_id]
			.classes
			.iter()
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				let selector = self.groups[class_id]
					.selector
					.as_ref()
					.expect("dynamic classes should have selectors");
				let rules = self.apply_all(class_id);
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
				}
			})
			.collect::<TokenStream>();
		let queue = self.queue_classes(group_id);
		quote! {
			#apply
			#queue
		}
	}

	pub fn apply_listeners(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.listeners
			.iter()
			.map(|&listener_id| {
				let rules = self.apply_all(listener_id);
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
							CLASSES.with(|classes| {
								e.stop_propagation();
								let window = web_sys::window().expect("getting window");
								let document = window.document().expect("getting `window.document`");
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

	fn apply_elements(&self, group_id: usize) -> TokenStream {
		if self.groups[group_id].elements.len() > 0 {
			let elements = self.groups[group_id].elements.iter().map(|&element_id| {
				let rules = self.apply_all(element_id);
				let tag = self.groups[element_id].tag();
				let class_names = &self.groups[element_id].class_names;
				quote! {
					element.append_child({
						let mut element = create_element(#tag, vec![#( #class_names ),*], &classes);
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
		} else {
			quote! {}
		}
	}

	fn apply_properties(&self, group_id: usize) -> TokenStream {
		let properties = &self.groups[group_id].properties;
		let mut effects = Vec::new();
		if let Some(value) = properties.cwl.get(&CwlProperty::Text) {
			effects.push(quote! { element.text(#value); });
		}
		if let Some(_value) = properties.cwl.get(&CwlProperty::Link) {
			effects.push(quote! {});
		}

		for (property, value) in &properties.css {
			let property = property.css();
			effects.push(quote! { element.css(#property, #value); });
		}

		quote! { #( #effects )* }
	}
}

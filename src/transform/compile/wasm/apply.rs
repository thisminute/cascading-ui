use {
	data::semantics::{properties::CwlProperty, Semantics},
	proc_macro2::TokenStream,
	quote::quote,
	transform::compile::css::Css,
};

impl Semantics {
	fn apply_all(&self, group_id: usize) -> TokenStream {
		let classes = self.apply_classes(group_id);
		let listeners = self.apply_listeners(group_id);
		let elements = self.apply_element(group_id);
		let properties = self.apply_properties(group_id);
		quote! {
			#classes
			#listeners
			#elements
			#properties
		}
	}

	pub fn apply_classes(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.classes
			.iter()
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				eprintln!("{} {}", group_id, class_id);
				let selector = self.groups[class_id]
					.selector
					.as_ref()
					.expect("dynamic classes should have a selector");
				let rules = self.apply_all(class_id);
				quote! {
					for element in document.get_elements_by_class_name(#selector) {
						#rules
					}
				}
			})
			.collect()
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
					"click" => quote! { set_onclick },
					"mouseover" => quote! { set_onmouseover },
					_ => panic!("unknown event id"),
				};
				quote! {
					let closure = {
						let element = element.clone();
						Closure::wrap(Box::new(move |_e: Event| {
							let window = web_sys::window().expect("getting window");
							let document = window.document().expect("getting `window.document`");
							#rules
						}) as Box<dyn FnMut(Event)>)
					};
					element.#event(Some(closure.as_ref().unchecked_ref()));
					closure.forget();
				}
			})
			.collect()
	}

	fn apply_element(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.elements
			.iter()
			.map(|&listener_id| {
				let rules = self.apply_all(listener_id);
				let tag = self.groups[listener_id].tag();
				quote! {
					element.append_child({
						let element = document
							.create_element(#tag)
							.expect(&*format!("Failed to create `{}` element.", #tag))
							.dyn_into::<HtmlElement>()
							.unwrap();
						#rules
						element
					}).unwrap();
				}
			})
			.collect()
	}

	fn apply_properties(&self, group_id: usize) -> TokenStream {
		let properties = &self.groups[group_id].properties;
		eprintln!("applying properties of group {}", group_id);
		let mut effects = Vec::new();
		if let Some(text) = properties.cwl.get(&CwlProperty::Text) {
			effects.push(quote! {
				if let Some(element) = element
					.child_nodes()
					.item(0)
				{
					element.set_node_value(None);
				}
				element.prepend_with_str_1(#text).unwrap();
			});
		}
		if let Some(_link) = properties.cwl.get(&CwlProperty::Link) {
			effects.push(quote! {});
		}

		for (property, value) in &properties.css {
			let property = property.css();
			effects.push(quote! {
				element.style().set_property(#property, #value).unwrap();
			});
		}

		quote! { #( #effects )* }
	}
}

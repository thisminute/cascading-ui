use {
	data::semantics::{Semantics, Value},
	proc_macro2::TokenStream,
	quote::quote,
	std::convert::TryInto,
};

// these functions generate code to process elements and classes which have
// been partially or fully compiled and rendered into html and css. should
// walk the tree and generate no code unless listeners are encountered

impl Semantics {
	pub fn compiled_element(&self, element_id: usize) -> TokenStream {
		let elements =
			(self.groups[element_id].elements.iter())
				.enumerate()
				.map(|(i, &child_id)| {
					let rules = self.compiled_element(child_id);
					if rules.is_empty() {
						return quote! {};
					}

					let i: u32 = i.try_into().unwrap();
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
				});

		let classes = (self.groups[element_id].classes.iter())
			.flat_map(|(_, groups)| groups.iter())
			.filter(|&&class_id| self.groups[class_id].is_dynamic)
			.map(|&class_id| {
				let rules = self.compiled_register_group(class_id);
				if rules.is_empty() {
					return quote! {};
				}

				let selector = self.groups[class_id].selector.clone().unwrap();
				quote! {
					{
						let mut group = classes.entry(#selector).or_default();
						#rules
					}
				}
			});

		let listeners = self.compiled_listeners(element_id);

		let variables = (self.groups[element_id].properties.iter())
			.filter_map(|(property, value)| {
				if let Value::Variable(variable_id, _) = value {
					if let (_, Some(mutable_id)) = self.variables[*variable_id] {
						return Some((property, mutable_id));
					}
				}
				None
			})
			.map(|(property, mutable_id)| {
				quote! {
					state[#mutable_id].1.push(
						Effect {
							property: #property,
							target: EffectTarget::Element(element.clone()),
						}
					);
				}
			});

		quote! {
			#( #elements )*
			#( #classes )*
			#listeners
			#( #variables )*
		}
	}

	pub fn compiled_listeners(&self, group_id: usize) -> TokenStream {
		(self.groups[group_id].listeners.iter())
			.map(|&listener_id| {
				let rules = self.compiled_dynamic_group(listener_id);
				if rules.is_empty() {
					return quote! {};
				}

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

				let rules = self.provide_state(rules);
				quote! {
					let closure = {
						let document = document.clone();
						let mut element = element.clone();
						Closure::wrap(Box::new(move |e: Event| {
							e.stop_propagation();
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
}

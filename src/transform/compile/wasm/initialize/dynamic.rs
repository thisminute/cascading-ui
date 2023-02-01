use {
	data::semantics::{
		properties::{CuiProperty, Property},
		Semantics, Value,
	},
	proc_macro2::TokenStream,
	quote::quote,
};

impl Semantics {
	pub fn compiled_dynamic_group(&self, group_id: usize) -> TokenStream {
		let elements = {
			let elements = (self.groups[group_id].elements.iter()).map(|&element_id| {
				let tag = self.groups[element_id].tag;
				let class_names = &self.groups[element_id].class_names;
				let element = if !class_names.is_empty() {
					quote! {
						let mut element = document
							.create_element(#tag)
							.unwrap()
							.dyn_into::<HtmlElement>()
							.unwrap();
						let class_names = vec![#( #class_names ),*];
						element.set_class_name(&*class_names.join(" "));
						for class_name in class_names {
							if let Some(source) = classes.get(class_name) {
								// TODO: avoid cloning?
								let source = &source.clone();
								render_elements(source, &mut element, &mut classes);
								render_listeners(source, &mut element);
								render_properties(source, &mut element);
							}
						}
						element
					}
				} else {
					quote! {
						document
							.create_element(#tag)
							.unwrap()
							.dyn_into::<HtmlElement>()
							.unwrap()
					}
				};
				let rules = self.compiled_dynamic_group(element_id);
				quote! {
					element.append_child({
						let mut element = {
							#element
						};
						#rules
						&element.into()
					}).unwrap();
				}
			});

			if elements.len() == 0 {
				quote! {}
			} else {
				quote! {
					while let Some(child) = element.last_element_child() {
						element.remove_child(&child.dyn_into::<Node>().unwrap()).unwrap();
					}

					#( #elements )*
				}
			}
		};
		let classes = (self.groups[group_id].classes.iter())
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				let rules = self.compiled_dynamic_group(class_id);
				let queue = self.compiled_register_group(class_id);
				if !self.groups[class_id].is_dynamic || rules.is_empty() {
					return quote! {};
				}
				let selector = self.groups[class_id]
					.selector
					.as_ref()
					.expect("dynamic classes should have selectors");
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
					let mut group = classes.entry(#selector).or_default();
					#queue
				}
			});
		let listeners = (self.groups[group_id].listeners.iter()).map(|&listener_id| {
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
		});
		let properties = self.compiled_dynamic_properties(group_id);
		quote! {
			#elements
			#( #classes )*
			#( #listeners )*
			#properties
		}
	}

	// fn compiled_render_variables(&self, group_id: usize) -> TokenStream {
	// 	self.groups[group_id]
	// 		.variables
	// 		.iter()
	// 		.map(|(_, _)| {
	// 			// let value = self.get_static(&value).to_string();

	// 			quote! {}
	// 		})
	// 		.collect()
	// }

	fn compiled_dynamic_properties(&self, group_id: usize) -> TokenStream {
		let properties = &self.groups[group_id].properties;
		let mut effects = Vec::new();

		if let Some(value) = properties.get(&Property::Cui(CuiProperty::Text)) {
			let value = self.compiled_dynamic_value(value);
			effects.push(quote! { element.text(#value); });
		}

		// if let Some(_value) = properties.get(&Property::Cui(CuiProperty::Link)) {
		// 	effects.push(quote! {});
		// }

		for (property, value) in properties {
			if let Property::Css(property) = property {
				effects.push(quote! { element.css(#property, #value); });
			}
		}

		effects.into_iter().collect()
	}

	fn compiled_dynamic_value(&self, value: &Value) -> TokenStream {
		if let &Value::Variable(variable_id, _) = value {
			if let (_, Some(mutable_id)) = self.variables[variable_id] {
				return quote! { state[#mutable_id] };
			}
		}
		quote! { #value }
	}
}

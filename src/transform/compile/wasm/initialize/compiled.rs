use {
	data::semantics::{Semantics, StaticValue},
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
			.map(|&class_id| {
				let rules = self.compiled_register_group(class_id);
				if !self.groups[class_id].is_dynamic || rules.is_empty() {
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
		let listeners = (self.groups[element_id].listeners.iter()).map(|&listener_id| {
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
		quote! {
			#( #elements )*
			#( #classes )*
			#( #listeners )*
		}
	}

	pub fn compiled_variables(&self) -> TokenStream {
		if self.mutable_count == 0 {
			return quote! {};
		}

		let mut mutables = vec![quote! {}; self.mutable_count];
		for (value, mutable_id) in &self.variables {
			if let &Some(mutable_id) = mutable_id {
				if !mutables[mutable_id].is_empty() {
					panic!("")
				}
				let type_ = match self.get_static(value) {
					StaticValue::Number(_) => quote! { Number },
					StaticValue::String(_) => quote! { String },
					// StaticValue::Color(_, _, _, _) => quote! { String },
				};
				mutables[mutable_id] = quote! {
					Value::#type_(#value),
				};
			}
		}
		quote! {
			thread_local! {
				static STATE: RefCell<Vec<Value>> = RefCell::new(vec![
					#(#mutables)*
				]);
			}
		}
	}

	pub fn provide_state(&self, tokens: TokenStream) -> TokenStream {
		if self.mutable_count == 0 {
			tokens
		} else {
			quote! {
				STATE.with(|state| {
					let mut state = state.borrow_mut();
					#tokens
				});
			}
		}
	}
}

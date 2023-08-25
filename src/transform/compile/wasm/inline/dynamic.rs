use {
	data::semantics::{
		properties::{CuiProperty, Property},
		Semantics,
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
			.filter(|&&class_id| self.groups[class_id].is_dynamic)
			.map(|&class_id| {
				let rules = self.compiled_dynamic_group(class_id);
				let queue = self.compiled_register_group(class_id);
				if rules.is_empty() {
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

		let listeners = self.compiled_listeners(group_id);

		let properties = (self.groups[group_id].properties.iter()).map(|(property, value)| {
			let value = self.dynamic_value(value);
			match property {
				Property::Css(property) => {
					quote! { element.css(#property, #value); }
				}
				Property::Cui(property) => match property {
					CuiProperty::Text => quote! { element.text(#value); },
					CuiProperty::Link => quote! {},
					CuiProperty::Tooltip => quote! {},
					CuiProperty::Image => quote! {},
				},
				Property::Page(_) => quote! {},
			}
		});

		let variables = (self.groups[group_id].variables.iter())
			.filter_map(|(_, variable_id)| {
				if let (value, Some(mutable_id)) = &self.variables[*variable_id] {
					Some((value, mutable_id))
				} else {
					None
				}
			})
			.map(|(value, mutable_id)| {
				let value = self.initial_value(value);
				quote! {
					state[#mutable_id].0 = #value;
					for Effect { property, target } in &state[#mutable_id].1 {
						if let EffectTarget::Element(element) = target {
							render_property(element, property, #value);
						}
					}
				}
			});

		quote! {
			#( #variables )*

			#elements
			#( #classes )*
			#listeners
			#( #properties )*
		}
	}
}

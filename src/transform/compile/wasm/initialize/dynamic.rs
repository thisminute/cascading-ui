use crate::data::semantics::StaticValue;

use {
	crate::data::semantics::{
		properties::{CuiProperty, Property},
		Semantics, Value,
	},
	proc_macro2::TokenStream,
	quote::{format_ident, quote},
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

		let properties = self.compiled_dynamic_properties(group_id);

		let apply = self.compiled_apply_call(group_id);

		let variables = (self.groups[group_id].variables.iter())
			.chain(self.groups[group_id].assignments.iter())
			.filter_map(|(_, variable_id)| {
				if let (value, Some(mutable_id)) = &self.variables[*variable_id] {
					Some((value, mutable_id))
				} else {
					None
				}
			})
			.map(|(value, mutable_id)| {
				let type_ = match self.get_static(value) {
					StaticValue::Number(_) => quote! { Number },
					StaticValue::String(_) => quote! { String },
				};
				let value = quote! {
					Value::#type_(#value)
				};

				let persist_save = if let Some(key) = self.persistent_mutables.get(mutable_id) {
					quote! {
						if let Value::String(s) = &state[#mutable_id].0 {
							let window = web_sys::window().unwrap();
							if let Ok(Some(storage)) = window.local_storage() {
								storage.set_item(#key, s).ok();
							}
						}
					}
				} else {
					quote! {}
				};

				quote! {
					state[#mutable_id].0 = #value;
					#persist_save
					for Effect { property, target } in &state[#mutable_id].1 {
						match target {
							EffectTarget::Element(element) => {
								render_property(element, property, state[#mutable_id].0.clone());
							}
							EffectTarget::Class(class_name) => {
								let window = web_sys::window().unwrap();
								let document = window.document().unwrap();
								let elements = document.get_elements_by_class_name(class_name);
								for i in 0..elements.length() {
									let element = elements
										.item(i)
										.unwrap()
										.dyn_into::<HtmlElement>()
										.unwrap();
									render_property(&element, property, state[#mutable_id].0.clone());
								}
							}
						}
					}
				}
			});

		quote! {
			#( #variables )*

			#elements
			#( #classes )*
			#listeners
			#properties
			#apply
		}
	}

	/// If the group has an `apply: .class_name` property, generate a call to
	/// the corresponding apply function.
	fn compiled_apply_call(&self, group_id: usize) -> TokenStream {
		let apply_prop = self.groups[group_id]
			.properties
			.get(&Property::Cui(CuiProperty::Apply));

		if let Some(Value::ClassRef(class_name)) = apply_prop {
			let fn_name = format_ident!("apply_{}", class_name);
			if self.mutable_count > 0 {
				quote! {
					#fn_name(&mut element, &document, &mut *classes, &mut *state);
				}
			} else {
				quote! {
					#fn_name(&mut element, &document, &mut *classes);
				}
			}
		} else {
			quote! {}
		}
	}

	pub(crate) fn compiled_dynamic_properties(&self, group_id: usize) -> TokenStream {
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
				let value = self.compiled_dynamic_value(value);
				effects.push(quote! { element.css(#property, #value); });
			}
		}
		effects.into_iter().collect()
	}

	fn compiled_dynamic_value(&self, value: &Value) -> TokenStream {
		if let &Value::Variable(variable_id, _) = value {
			if let (_, Some(mutable_id)) = self.variables[variable_id] {
				return quote! { state[#mutable_id].0.clone() };
			}
		}
		// Static value — construct the runtime Value enum
		let type_ = match self.get_static(value) {
			StaticValue::Number(_) => quote! { Number },
			StaticValue::String(_) => quote! { String },
		};
		quote! { Value::#type_(#value) }
	}
}

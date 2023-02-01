use {
	data::semantics::{properties::Property, Semantics},
	proc_macro2::TokenStream,
	quote::quote,
};

// the generated code to register the effects of listeners and classes in the
// runtime, to be used when a listener is triggered

impl Semantics {
	pub fn compiled_register_group(&self, group_id: usize) -> TokenStream {
		let elements = (self.groups[group_id].elements.iter())
			.map(|&element_id| {
				let rules = self.compiled_register_group(element_id);
				quote! {
					group.elements.push({
						let mut group = Group::default();
						#rules
						group
					});
				}
			})
			.collect::<TokenStream>();
		let classes = (self.groups[group_id].classes.iter())
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				let rules = self.compiled_register_group(class_id);
				if rules.is_empty() || self.groups[class_id].selector.is_none() {
					return quote! {};
				}
				let selector = self.groups[class_id].selector.clone().unwrap();
				quote! {
					group.classes.push({
						let mut group = Group::default();
						group.selector = #selector;
						#rules
						group
					});
				}
			})
			.collect::<TokenStream>();
		let listeners = (self.groups[group_id].listeners.iter())
			.map(|&listener_id| {
				let rules = self.compiled_register_group(listener_id);
				if rules.is_empty() {
					return quote! {};
				}
				let selector = self.groups[listener_id]
					.name
					.clone()
					.expect("listeners should have event names");
				quote! {
					group.listeners.push({
						let mut group = Group::default();
						group.selector = #selector;
						#rules
						group
					});
				}
			})
			.collect::<TokenStream>();
		let properties = (self.groups[group_id].properties.iter())
			.map(|(property, value)| match property {
				Property::Css(property) => quote! {
					group.properties.insert(Property::Css(#property), #value);
				},
				Property::Cui(property) => quote! {
					group.properties.insert(Property::#property, #value);
				},
				_ => panic!("aaaAA"),
			})
			.collect::<TokenStream>();
		quote! {
			#elements
			#classes
			#listeners
			#properties
		}
	}
}

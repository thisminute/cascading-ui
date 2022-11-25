use {
	data::semantics::{properties::Property, Semantics},
	proc_macro2::TokenStream,
	quote::quote,
};

impl Semantics {
	pub fn static_register_all(&self, class_id: usize) -> TokenStream {
		let elements = self.static_register_elements(class_id);
		let classes = self.static_register_classes(class_id);
		let listeners = self.static_register_listeners(class_id);
		// let variables = self.static_register_variables(class_id);
		let properties = self.static_register_properties(class_id);
		quote! {
			#elements
			#classes
			#listeners
			// #variables
			#properties
		}
	}

	fn static_register_elements(&self, class_id: usize) -> TokenStream {
		self.groups[class_id]
			.elements
			.iter()
			.map(|&element_id| {
				let rules = self.static_register_all(element_id);
				quote! {
					class.elements.push({
						let mut class = Group::default();
						#rules
						class
					});
				}
			})
			.collect()
	}

	fn static_register_classes(&self, group_id: usize) -> TokenStream {
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
					class.classes.push({
						let mut class = Group::default();
						class.selector = #selector;
						#rules
						class
					});
				}
			})
			.collect()
	}

	fn static_register_listeners(&self, class_id: usize) -> TokenStream {
		self.groups[class_id]
			.listeners
			.iter()
			.map(|&listener_id| {
				let selector = self.groups[listener_id]
					.name
					.clone()
					.expect("listeners should have event names");
				let rules = self.static_register_all(listener_id);
				quote! {
					class.listeners.push({
						let mut class = Group::default();
						class.selector = #selector;
						#rules
						class
					});
				}
			})
			.collect()
	}

	// fn static_register_variables(&self, class_id: usize) -> TokenStream {
	// 	self.groups[class_id]
	// 		.variables
	// 		.iter()
	// 		.map(|(identifier, value)| {
	// 			let identifier = cui_ident(identifier);
	// 			quote! { let mut #identifier = #value; }
	// 		})
	// 		.collect()
	// }

	fn static_register_properties(&self, class_id: usize) -> TokenStream {
		self.groups[class_id]
			.properties
			.iter()
			.map(|(property, value)| match property {
				Property::Css(property) => quote! {
					class.properties.insert(Property::Css(#property), #value);
				},
				Property::Cui(property) => quote! {
					class.properties.insert(Property::#property, #value);
				},
				_ => quote! {},
			})
			.collect()
	}
}

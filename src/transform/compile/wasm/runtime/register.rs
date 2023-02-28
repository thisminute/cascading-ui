use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

impl Semantics {
	pub fn runtime_register_functions() -> TokenStream {
		quote! {
			fn register_classes(source: &Group, classes: &mut HashMap<&'static str, Group>) {
				for source in &source.classes {
					for class in &source.classes {
						register_classes(class, classes);
					}
					let mut target = classes.entry(source.selector).or_default();
					if !source.elements.is_empty() {
						target.elements = Vec::new();
						for element in &source.elements {
							target.elements.push(element.clone());
						}
					}
					for listener in &source.listeners {
						target.listeners.push(listener.clone());
					}
					for (property, value) in source.properties.clone() {
						target.properties.insert(property, value);
					}
				}
			}
		}
	}
}

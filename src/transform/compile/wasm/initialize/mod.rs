mod compiled;
mod dynamic;
mod register;

use {crate::data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

// entry point for wasm compilation

impl Semantics {
	pub fn document(&self) -> TokenStream {
		let rules = self.compiled_element(self.pages[0].root_id);
		if rules.is_empty() {
			return quote! {};
		}

		let persistent_apply = self.compiled_persistent_apply();

		quote! {
			CLASSES.with(|classes| {
				let mut classes = classes.borrow_mut();
				let window = web_sys::window().expect("getting window");
				let document = &window.document().expect("getting `window.document`");
				let head = &document.head().expect("getting `window.document.head`");
				let body = &document.body().expect("getting `window.document.body`");

				let element = body
					.children()
					.item(0)
					.expect("body should have a root element")
					.dyn_into::<HtmlElement>()
					.unwrap();
				#rules
				#persistent_apply
			});
		}
	}

	/// After all effects are registered, apply current values for persistent
	/// variables. These may differ from compile-time defaults if localStorage
	/// had a stored value.
	fn compiled_persistent_apply(&self) -> TokenStream {
		let applies: Vec<TokenStream> = self
			.persistent_mutables
			.keys()
			.map(|mutable_id| {
				quote! {
					for Effect { property, target } in &state[#mutable_id].1 {
						match target {
							EffectTarget::Element(element) => {
								render_property(element, property, state[#mutable_id].0.clone());
							}
							EffectTarget::Class(class_name) => {
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
			})
			.collect();

		if applies.is_empty() {
			quote! {}
		} else {
			quote! { #( #applies )* }
		}
	}
}

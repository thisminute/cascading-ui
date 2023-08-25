mod api;
mod inline;
mod value;

use {
	data::semantics::Semantics,
	proc_macro2::{Span, TokenStream},
	quote::{quote, quote_spanned},
};

impl Semantics {
	pub fn get_mutables(&self) -> TokenStream {
		let mut mutables = vec![quote! {}; self.mutable_count];
		for (value, mutable_id) in &self.variables {
			if let &Some(mutable_id) = mutable_id {
				let initial_value = self.initial_value(value);
				mutables[mutable_id] = quote! {
					Value::#initial_value
				};
			}
		}
		quote! { vec![ #( (#mutables, Vec::new()), )* ] }
	}

	pub fn compiled_variables(&self) -> TokenStream {
		if self.mutable_count == 0 {
			return quote! {};
		}
		let mutables = self.get_mutables();
		quote! {
			thread_local! {
				static STATE: RefCell<Vec<(Value, Vec<Effect>)>> = RefCell::new(#mutables);
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

	pub fn wasm(&self, full: bool) -> TokenStream {
		log::debug!("...Writing Wasm...");

		let warnings = self.warnings.iter().map(|error| {
			quote_spanned! {Span::call_site()=>
				compile_error!(#error);
			}
		});
		let errors = self.errors.iter().map(|error| {
			quote_spanned! {Span::call_site()=>
				compile_error!(#error);
			}
		});

		let document = self.document();

		log::debug!("Generated document");

		if document.is_empty() {
			return quote! {};
		}

		let document = self.provide_state(document);

		let core = if !self.errors.is_empty() {
			quote! {
				#( #errors )*
			}
		} else if full {
			self.full(document)
		} else {
			document
		};

		quote! {
			#( #warnings )*
			#core
		}
	}

	fn full(&self, document: TokenStream) -> TokenStream {
		let header = Self::runtime();
		let state = self.compiled_variables();
		quote! {
			#header
			#state

			extern crate console_error_panic_hook;

			#[wasm_bindgen(start)]
			pub fn run() -> Result<(), JsValue> {
				std::panic::set_hook(Box::new(console_error_panic_hook::hook));
				#document
				Ok(())
			}
		}
	}
}

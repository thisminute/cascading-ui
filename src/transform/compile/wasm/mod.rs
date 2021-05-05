mod apply;
mod initialize;
mod queue;

use {
	data::semantics::Semantics,
	proc_macro2::{Span, TokenStream},
	quote::{quote, quote_spanned},
};

impl Semantics {
	pub fn wasm(&self, full: bool) -> TokenStream {
		eprintln!("...Writing Wasm...");

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

		let core = if !self.errors.is_empty() {
			quote! {
				#( #errors )*
			}
		} else {
			if full {
				self.full(Self::header(), self.document())
			} else {
				self.document()
			}
		};

		quote! {
			#( #warnings )*
			#core
		}
	}

	fn full(&self, header: TokenStream, document: TokenStream) -> TokenStream {
		quote! {
			#header

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

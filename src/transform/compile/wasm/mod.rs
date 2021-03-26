mod runtime;

use {
	data::semantics::Semantics,
	proc_macro2::{Span, TokenStream},
	quote::{quote, quote_spanned},
};

impl Semantics {
	pub fn wasm(&self, full: bool) -> TokenStream {
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

	pub fn header() -> TokenStream {
		let web_sys_includes = vec![
			quote! { console },
			quote! { Document },
			// quote! { Element },
			quote! { Event },
			quote! { EventListener },
			// quote! { EventTarget },
			quote! { HtmlElement },
			// quote! { HtmlHeadElement },
			// quote! { Window },
		];

		quote! {
			extern crate wasm_bindgen;
			extern crate web_sys;
			use {
				std::collections::HashMap,
				wasm_bindgen::{
					prelude::*,
					JsCast,
					JsValue,
				},
				web_sys::{
					#( #web_sys_includes ),*
				},
			};

			#[derive(Clone, Hash)]
			pub enum Property {
				Css(&'static str),
				Link,
				Text,
				Tooltip,
				Image,
			}
			impl PartialEq for Property {
				fn eq(&self, other: &Self) -> bool {
					match self {
						Self::Css(a) => match other {
							Self::Css(b) => a == b,
							_ => false,
						},
						a => match other {
							Self::Css(_) => false,
							b => a == b,
						},
					}
				}
			}
			impl Eq for Property {}

			struct Rule {
				properties: HashMap<Property, &'static str>,
				elements: Vec<Rule>,
			}

			impl Rule {
				fn cascade(&mut self, rule: &Rule){
					for (property, value) in &rule.properties {
						let property = property.clone();
						self.properties.insert(property, value);
					}
				}
			}
		}
	}
}

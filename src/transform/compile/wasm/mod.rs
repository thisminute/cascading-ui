mod initialize;
mod runtime;

use {
	data::semantics::Semantics,
	proc_macro2::{Span, TokenStream},
	quote::{quote, quote_spanned},
};

impl Semantics {
	pub fn runtime() -> TokenStream {
		let header = Self::header();
		let runtime_register_functions = Self::runtime_register_functions();
		let runtime_render_functions = Self::runtime_render_functions();
		let runtime_static_render_functions = Self::runtime_static_render_functions();
		quote! {
			#header
			#runtime_register_functions
			#runtime_render_functions
			#runtime_static_render_functions
		}
	}

	fn header() -> TokenStream {
		quote! {
			extern crate wasm_bindgen;
			extern crate web_sys;
			use {
				std::{
					cell::RefCell,
					collections::HashMap,
				},
				wasm_bindgen::{
					prelude::*,
					JsCast,
					JsValue,
				},
				web_sys::{
					console,
					Event,
					HtmlElement,
					Node,
				},
			};

			#[derive(Clone, Hash, PartialEq, Eq)]
			pub enum Property {
				Css(&'static str),
				Link,
				Text,
				Tooltip,
				Image,
			}

			#[derive(Clone, Default)]
			struct Group {
				class_names: Vec<&'static str>,
				selector: &'static str,

				classes: Vec<Group>,
				listeners: Vec<Group>,
				elements: Vec<Group>,
				properties: HashMap<Property, &'static str>,
			}

			trait Std {
				fn text(&mut self, value: &'static str);
				fn css(&mut self, property: &'static str, value: &'static str);
			}

			impl Std for HtmlElement {
				fn text(&mut self, value: &'static str) {
					if let Some(element) = self
						.child_nodes()
						.item(0)
					{
						element.set_node_value(None);
					}
					self.prepend_with_str_1(value).unwrap();
				}

				fn css(&mut self, property: &'static str, value: &'static str) {
					self.style().set_property(property, value).unwrap();
				}
			}

			thread_local! {
				static CLASSES: RefCell<HashMap<&'static str, Group>> = RefCell::new(HashMap::new());
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

		let core = if !self.errors.is_empty() {
			quote! {
				#( #errors )*
			}
		} else {
			if full {
				self.full()
			} else {
				self.document()
			}
		};

		quote! {
			#( #warnings )*
			#core
		}
	}

	fn full(&self) -> TokenStream {
		let header = Self::runtime();
		let document = self.document();
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

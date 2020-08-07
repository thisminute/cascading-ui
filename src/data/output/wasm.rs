use {
	data::{
		context::Context,
		meta::Meta,
		tokens::{Block, Document, Lib, Prefix, Rule, Website},
	},
	syn::export::{quote::quote, TokenStream2},
};

pub trait Wasm {
	fn wasm(&self, meta: &Meta, context: Option<&Context>) -> TokenStream2;
}

impl Wasm for Website {
	fn wasm(&self, meta: &Meta, context: Option<&Context>) -> TokenStream2 {
		let lib = Lib {}.wasm(meta, context);
		let builder = self.document.wasm(meta, context);

		quote! {
			#lib

			#[wasm_bindgen(start)]
			pub fn run() -> Result<(), JsValue> {
				#builder
				Ok(())
			}
		}
	}
}

impl Wasm for Lib {
	fn wasm(&self, _meta: &Meta, _context: Option<&Context>) -> TokenStream2 {
		quote! {
			extern crate wasm_bindgen;
			extern crate web_sys;
			use {
				wasm_bindgen::{
					prelude::*,
					JsCast,
				},
				web_sys::{
					Document,
					Event,
					HtmlElement,
					Window,
				},
			};
			fn create_element(document: &Document, name: &str) -> HtmlElement {
				document
					.create_element(name)
					.expect(&format!("Failed to create `{}` element.", name)[..])
					.dyn_into::<HtmlElement>()
					.expect("Failed to construct element.")
			}
		}
	}
}

impl Wasm for Document {
	fn wasm(&self, meta: &Meta, _context: Option<&Context>) -> TokenStream2 {
		let warnings = &meta.warnings;
		if !meta.errors.is_empty() {
			let errors = &meta.errors;
			return quote! {
				#( #warnings )*
				#( #errors )*
			};
		}

		let body = self.root.wasm(meta, None);
		quote! {
			#( #warnings )*

			use {
				web_sys::{
					Element,
					HtmlHeadElement,
				},
			};

			struct Meta<'a> {
				window: Window,
				document: &'a Document,
				head: &'a HtmlHeadElement,
				style: &'a Element,
			}

			let window = web_sys::window().expect("getting window");
			let document = &window.document().expect("getting `window.document`");
			let head = &document.head().expect("getting `window.document.head`");
			let body = document.body().expect("getting `window.document.body`");
			let style = &document.create_element("style").expect("creating a `style` element");
			head.append_child(style).expect("appending `style` to `head`");
			let current_element = &body;
			let meta = Meta { window, document, head, style };
			#body
		}
	}
}

impl Wasm for Block {
	fn wasm(&self, meta: &Meta, context: Option<&Context>) -> TokenStream2 {
		let identifier = &self.identifier.to_string()[..];

		match self.prefix {
			Prefix::Instance => {
				let rule_quotes = self.rules.iter().map(|rule| rule.wasm(meta, context));
				let block_quotes = self.blocks.iter().map(|block| block.wasm(meta, context));

				let quotes = quote! {
					#( #rule_quotes )*
					#( #block_quotes )*
				};

				match identifier {
					_ => {
						quote! {
							let element = &create_element(&meta.document, #identifier);
							current_element.append_child(element).unwrap();
							let current_element = element;

							#quotes

							let current_element = current_element.parent_element().unwrap();
						}
					}
				}
			}
			Prefix::Class => {
				quote! {}
			}
			Prefix::Action => {
				quote! {}
			}
			Prefix::Listener => {
				quote! {}
			}
		}
	}
}

impl Wasm for Rule {
	fn wasm(&self, _meta: &Meta, _context: Option<&Context>) -> TokenStream2 {
		let property = &self.property.to_string();
		let value = &self.value;

		match &property.to_string()[..] {
			"text" => {
				quote! {
					current_element.set_inner_html(#value);
				}
			}
			"link" => {
				quote! {
					let on_click = Closure::wrap(Box::new(|_e: Event| {
						let window = web_sys::window().expect("getting window");
						let document = window.document().expect("getting `window.document`");
						document.location().unwrap().assign(#value).unwrap();
					}) as Box<dyn FnMut(Event)>);
					current_element.set_onclick(Some(on_click.as_ref().unchecked_ref()));
					current_element.style().set_property("cursor", "pointer").unwrap();
					on_click.forget();
				}
			}
			"tip" => {
				quote! {
					current_element.set_attribute("title", #value).unwrap();
				}
			}
			_ => {
				quote! {
					body.style().set_property(
						&str::replace(#property, "_", "-"),
						#value
					).unwrap();
				}
			}
		}
	}
}

use {
	data::{dom::Element, Semantics},
	syn::export::{quote::quote, quote::quote_spanned, Span, TokenStream2},
};

pub trait Wasm {
	fn wasm(&self) -> TokenStream2;
	fn website(lib: TokenStream2, builder: TokenStream2) -> TokenStream2;
	fn header() -> TokenStream2;
	fn document(&self) -> TokenStream2;
	fn element(&self, element: &Element) -> TokenStream2;
}

impl Wasm for Semantics<'_> {
	fn wasm(&self) -> TokenStream2 {
		quote! {}
	}
	fn website(lib: TokenStream2, builder: TokenStream2) -> TokenStream2 {
		quote! {
			#lib

			#[wasm_bindgen(start)]
			pub fn run() -> Result<(), JsValue> {
				#builder
				Ok(())
			}
		}
	}

	fn header() -> TokenStream2 {
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
			// fn create_element(document: &Document, name: &str) -> HtmlElement {
			// 	document
			// 		.create_element(name)
			// 		.expect(&format!("Failed to create `{}` element.", name)[..])
			// 		.dyn_into::<HtmlElement>()
			// 		.expect("Failed to construct element.")
			// }
		}
	}

	fn document(&self) -> TokenStream2 {
		let warnings = &self.warnings.iter().map(|error| {
			quote_spanned! {Span::call_site()=>
				compile_error!(#error);
			}
		});
		if !self.errors.is_empty() {
			let errors = &self.errors.iter().map(|error| {
				quote_spanned! {Span::call_site()=>
					compile_error!(#error);
				}
			});

			return quote! {
				#( #warnings )*
				#( #errors )*
			};
		}

		let body = self.element(&self.dom);
		quote! {
			#( #warnings )*

			use {
				web_sys::{
					Element,
					HtmlHeadElement,
				},
			};

			let window = web_sys::window().expect("getting window");
			let document = &window.document().expect("getting `window.document`");
			let head = &document.head().expect("getting `window.document.head`");
			let body = document.body().expect("getting `window.document.body`");
			#body
		}
	}

	fn element(&self, _element: &Element) -> TokenStream2 {
		// if element.link
		quote! {}
		// let identifier = "div";

		// if element.active {
		// 	let rule_quotes = self.rules.iter().map(|rule| rule.wasm(semantics, context));
		// 	let block_quotes = self
		// 		.blocks
		// 		.iter()
		// 		.map(|block| block.wasm(semantics, context));

		// 	let quotes = quote! {
		// 		#( #rule_quotes )*
		// 		#( #block_quotes )*
		// 	};
		// }
	}

	// fn rule(&self) -> TokenStream2 {
	// 	let property = &self.property.to_string();
	// 	let value = &self.value;

	// 	match &property.to_string()[..] {
	// 		"text" => {
	// 			quote! {
	// 				current_element.set_inner_html(#value);
	// 			}
	// 		}
	// 		"link" => {
	// 			quote! {
	// 				let on_click = Closure::wrap(Box::new(|_e: Event| {
	// 					let window = web_sys::window().expect("getting window");
	// 					let document = window.document().expect("getting `window.document`");
	// 					document.location().unwrap().assign(#value).unwrap();
	// 				}) as Box<dyn FnMut(Event)>);
	// 				current_element.set_onclick(Some(on_click.as_ref().unchecked_ref()));
	// 				current_element.style().set_property("cursor", "pointer").unwrap();
	// 				on_click.forget();
	// 			}
	// 		}
	// 		"tip" => {
	// 			quote! {
	// 				current_element.set_attribute("title", #value).unwrap();
	// 			}
	// 		}
	// 		_ => {
	// 			quote! {
	// 				body.style().set_property(
	// 					&str::replace(#property, "_", "-"),
	// 					#value
	// 				).unwrap();
	// 			}
	// 		}
	// 	}
	// }
}

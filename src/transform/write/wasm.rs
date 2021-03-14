use {
	data::{
		dom::{Dom, Element},
		semantics::Semantics,
	},
	proc_macro2::{Span, TokenStream as TokenStream2},
	quote::{quote, quote_spanned},
};

pub trait Wasm {
	fn wasm(&self, semantics: &Semantics) -> TokenStream2;
	fn website(&self, header: TokenStream2, document: TokenStream2) -> TokenStream2;
	fn header(&self) -> TokenStream2;
	fn document(&self, semantics: &Semantics) -> TokenStream2;
	fn element(&self, element: &Element) -> TokenStream2;
}

impl Wasm for Dom {
	fn wasm(&self, semantics: &Semantics) -> TokenStream2 {
		if semantics.only_header_wasm {
			self.header()
		} else if semantics.bindgen {
			self.website(self.header(), self.document(semantics))
		} else {
			self.document(semantics)
		}
	}

	fn website(&self, header: TokenStream2, document: TokenStream2) -> TokenStream2 {
		quote! {
			#header

			#[wasm_bindgen(start)]
			pub fn run() -> Result<(), JsValue> {
				#document
				Ok(())
			}
		}
	}

	fn header(&self) -> TokenStream2 {
		let includes = vec![
			quote! { console },
			// quote! { Document },
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
				wasm_bindgen::{
					prelude::*,
					JsCast,
				},
				web_sys::{
					#( #includes ),*
				},
			};
			// fn create_element(document: &Document, name: &str) -> HtmlElement {
			// 	document
			// 		.create_element(name)
			// 		.expect(&*format!("Failed to create `{}` element.", name))
			// 		.dyn_into::<HtmlElement>()
			// 		.expect("Failed to construct element.")
			// }
		}
	}

	fn document(&self, semantics: &Semantics) -> TokenStream2 {
		let warnings = semantics.warnings.iter().map(|error| {
			quote_spanned! {Span::call_site()=>
				compile_error!(#error);
			}
		});
		if !semantics.errors.is_empty() {
			let errors = semantics.errors.iter().map(|error| {
				quote_spanned! {Span::call_site()=>
					compile_error!(#error);
				}
			});

			return quote! {
				#( #warnings )*
				#( #errors )*
			};
		}

		// let body = self.element(&self.dom[&0]);
		quote! {
			#( #warnings )*

			let window = web_sys::window().expect("getting window");
			let document = &window.document().expect("getting `window.document`");
			let head = &document.head().expect("getting `window.document.head`");
			let body = &document.body().expect("getting `window.document.body`");
		}
	}

	fn element(&self, element: &Element) -> TokenStream2 {
		let id = format!("aaa");
		let events = element.listeners.iter().map(|listener| {
				let event = quote! { set_onclick };
				// match listener.event {
				// 	Event::Click => quote! { set_onclick },
				// };

				let effects = vec![
					match &listener.effects.text {
						Some(text) => quote! { element.set_inner_html(#text); },
						None => quote! {},
					},
					match &listener.effects.link {
						Some(link) => {
							quote! { document.location().expect("a").assign(#link).expect("e"); }
						}
						None => quote! {},
					},
				];

				quote! {
					// EventTarget::new();

					let element = &document.get_element_by_id(#id).expect("asdf").dyn_into::<web_sys::HtmlElement>().expect("wqer");
					// EventListener::new(element, #event, move |_event| {
					// 	console::log_1(&"bbbbbbbbb".into());
					// 	element.set_inner_html("Hello World");
					// }).forget();
					let on_click = {
						let element = element.clone();
						Closure::wrap(Box::new(move |_e: Event| {
							let window = web_sys::window().expect("getting window");
							let document = window.document().expect("getting `window.document`");
							#( #effects )*
						}) as Box<dyn FnMut(Event)>)
					};
					element.#event(Some(on_click.as_ref().unchecked_ref()));
					element.style().set_property("cursor", "pointer").unwrap();
					on_click.forget();
				}
			});

		let children = element.children.iter().map(|child| self.element(child));

		quote! {
			#( #events )*
			#( #children )*
		}
	}

	// fn rule(&self) -> TokenStream2 {
	// 	let property = &self.property.to_string();
	// 	let value = &self.value;

	// 	match &*property.to_string() {
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

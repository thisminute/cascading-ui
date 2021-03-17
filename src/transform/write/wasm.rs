use {
	data::semantics::{
		properties::{CssProperty, CwlProperty},
		Group, Semantics,
	},
	proc_macro2::{Span, TokenStream},
	quote::{quote, quote_spanned},
};

pub trait Wasm {
	fn wasm(&self) -> TokenStream;
	fn website(&self, header: TokenStream, document: TokenStream) -> TokenStream;
	fn header(&self) -> TokenStream;
	fn document(&self) -> TokenStream;
	fn element(&self, element: &Group) -> TokenStream;
}

impl Wasm for Semantics {
	fn wasm(&self) -> TokenStream {
		if self.only_header_wasm {
			self.header()
		} else if self.bindgen {
			self.website(self.header(), self.document())
		} else {
			self.document()
		}
	}

	fn website(&self, header: TokenStream, document: TokenStream) -> TokenStream {
		quote! {
			#header

			#[wasm_bindgen(start)]
			pub fn run() -> Result<(), JsValue> {
				#document
				Ok(())
			}
		}
	}

	fn header(&self) -> TokenStream {
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
					JsValue,
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

	fn document(&self) -> TokenStream {
		let warnings = self.warnings.iter().map(|error| {
			quote_spanned! {Span::call_site()=>
				compile_error!(#error);
			}
		});
		if !self.errors.is_empty() {
			let errors = self.errors.iter().map(|error| {
				quote_spanned! {Span::call_site()=>
					compile_error!(#error);
				}
			});

			return quote! {
				#( #warnings )*
				#( #errors )*
			};
		}

		let runtime = self
			.pages
			.iter()
			.map(|&page| self.element(&self.groups[page]));

		quote! {
			#( #warnings )*

			let window = web_sys::window().expect("getting window");
			let document = &window.document().expect("getting `window.document`");
			let head = &document.head().expect("getting `window.document.head`");
			let body = &document.body().expect("getting `window.document.body`");

			#( #runtime )*
		}
	}

	fn element(&self, element: &Group) -> TokenStream {
		let events = element.listeners.iter().map(|&listener_id| {
			let listener = &self.groups[listener_id];
			let event = match &*listener
				.name
				.clone()
				.expect("listener should have an event id")
			{
				"click" => quote! { set_onclick },
				_ => panic!("unknown event id"),
			};

			let mut effects = Vec::new();
			if let Some(text) = listener.properties.cwl.get(&CwlProperty::Text) {
				effects.push(quote! { element.set_inner_html(#text); });
			}
			if let Some(link) = listener.properties.cwl.get(&CwlProperty::Link) {
				effects.push(quote! {
					document.location().expect("a").assign(#link).expect("e");
					element.style().set_property("cursor", "pointer").unwrap();
				});
			}
			let properties = listener.properties.css.iter().map(|(property, value)| {
				let property = match property {
					CssProperty::BackgroundColor => "background-color",
					CssProperty::Color => "color",
					CssProperty::Margin => "margin",
					CssProperty::Padding => "padding",
					CssProperty::Display => "display",
					CssProperty::Position => "position",
					CssProperty::Width => "width",
					CssProperty::Height => "height",
				};
				quote! {
					element.style().set_property(#property, #value).unwrap();
				}
			});

			let class = &listener.id;
			effects.push(quote! {
				let elements = document.get_elements_by_class_name(#class);
				for i in 0..elements.length() {
					let element = elements.item(i).unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
					#( #properties )*
				}
			});

			let class = listener
				.id
				.clone()
				.expect("listener should have a selector");
			quote! {
				let elements = document.get_elements_by_class_name(#class);
				console::log_1(&JsValue::from_str("aaaaaaaAA"));
				for i in 0..elements.length() {
					let element = elements.item(i).unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
					let on_click = {
						Closure::wrap(Box::new(move |_e: Event| {
							let window = web_sys::window().expect("getting window");
							let document = window.document().expect("getting `window.document`");
							#( #effects )*
						}) as Box<dyn FnMut(Event)>)
					};
					element.#event(Some(on_click.as_ref().unchecked_ref()));
					on_click.forget();
				}
			}
		});

		let children = element
			.elements
			.iter()
			.map(|&child_id| self.element(&self.groups[child_id]));

		quote! {
			#( #events )*
			#( #children )*
		}
	}

	// fn rule(&self) -> TokenStream {
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

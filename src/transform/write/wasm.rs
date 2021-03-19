use {
	data::{
		dom::{Dom, Element},
		semantics::properties::{CssProperty, CwlProperty},
	},
	proc_macro2::{Span, TokenStream},
	quote::{quote, quote_spanned},
};

impl Dom {
	pub fn wasm(
		&self,
		warnings: Vec<&'static str>,
		errors: Vec<&'static str>,
		full: bool,
	) -> TokenStream {
		let warnings = warnings.iter().map(|error| {
			quote_spanned! {Span::call_site()=>
				compile_error!(#error);
			}
		});
		let core = if !errors.is_empty() {
			let errors = errors.iter().map(|error| {
				quote_spanned! {Span::call_site()=>
					compile_error!(#error);
				}
			});

			quote! {
				#( #errors )*
			}
		} else {
			if full {
				self.website(Self::header(), self.document())
			} else {
				self.document()
			}
		};

		quote! {
			#( #warnings )*
			#core
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

	pub fn header() -> TokenStream {
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
			extern crate console_error_panic_hook;
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
		let runtime = self.html_roots.iter().map(|(_, page)| {
			let pages = page.root.children.iter().map(|page| page.element());
			quote! { #( #pages )* }
		});

		quote! {
			std::panic::set_hook(Box::new(console_error_panic_hook::hook));
			let window = web_sys::window().expect("getting window");
			let document = &window.document().expect("getting `window.document`");
			let head = &document.head().expect("getting `window.document.head`");
			let body = &document.body().expect("getting `window.document.body`");
			#( #runtime )*
		}
	}
}

impl Element {
	fn element(&self) -> TokenStream {
		let events = self.listeners.iter().map(|listener| {
			let mut effects = Vec::new();
			if let Some(text) = listener.properties.cwl.get(&CwlProperty::Text) {
				effects.push(quote! {
					element
						.child_nodes()
						.item(0)
						.unwrap()
						.set_node_value(Some(#text));
				});
			}
			if let Some(link) = listener.properties.cwl.get(&CwlProperty::Link) {
				effects.push(quote! {
					document.location().unwrap().assign(#link).unwrap();
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
			effects.push(quote! {
				#( #properties )*
			});

			let event = match &*listener
				.name
				.clone()
				.expect("every listener should have an event id")
			{
				"click" => quote! { set_onclick },
				_ => panic!("unknown event id"),
			};

			let class = &listener.id;
			quote! {
				let element = document
					.get_elements_by_class_name(#class)
					.item(0)
					.unwrap()
					.dyn_into::<web_sys::HtmlElement>()
					.unwrap();
				let on_click = {
					let element = element.clone();
					Closure::wrap(Box::new(move |_e: Event| {
						let window = web_sys::window().expect("getting window");
						let document = window.document().expect("getting `window.document`");
						#( #effects )*
					}) as Box<dyn FnMut(Event)>)
				};
				element.#event(Some(on_click.as_ref().unchecked_ref()));
				on_click.forget();
			}
		});

		let children = self.children.iter().map(|child| child.element());
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

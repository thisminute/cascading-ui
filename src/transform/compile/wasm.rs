use {
	data::semantics::{
		properties::{CssProperty, CwlProperty},
		Semantics,
	},
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
				wasm_bindgen::{
					prelude::*,
					JsCast,
					JsValue,
				},
				web_sys::{
					#( #web_sys_includes ),*
				},
			};
			fn create_element(document: &Document, name: &str) -> HtmlElement {
				document
					.create_element(name)
					.expect(&*format!("Failed to create `{}` element.", name))
					.dyn_into::<HtmlElement>()
					.unwrap()
			}
		}
	}

	fn document(&self) -> TokenStream {
		let listeners = self.pages.iter().map(|page| self.element(page.root_id));
		quote! {
			let window = web_sys::window().expect("getting window");
			let document = &window.document().expect("getting `window.document`");
			let head = &document.head().expect("getting `window.document.head`");
			let body = &document.body().expect("getting `window.document.body`");
			#( #listeners )*
		}
	}

	fn element(&self, element_id: usize) -> TokenStream {
		self.groups[element_id]
			.listeners
			.iter()
			.map(|(class, listener_id)| {
				let listener = &self.groups[*listener_id];
				let mut effects = Vec::new();
				if let Some(text) = listener.properties.cwl.get(&CwlProperty::Text) {
					effects.push(quote! {
						element
							.child_nodes()
							.item(0)
							.unwrap()
							.set_node_value(None);
						element.prepend_with_str_1(#text).unwrap();
					});
				}
				if let Some(link) = listener.properties.cwl.get(&CwlProperty::Link) {
					effects.push(quote! {
						document.location().unwrap().assign(#link).unwrap();
						element.style().set_property("cursor", "pointer").unwrap();
					});
				}

				for (property, value) in &listener.properties.css {
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
					effects.push(quote! {
						element.style().set_property(#property, #value).unwrap();
					});
				}
				for &child_id in &listener.elements {
					&self.groups[child_id];
					effects.push(quote! {});
				}

				let event = match &*listener
					.name
					.clone()
					.expect("every listener should have an event id")
				{
					"click" => quote! { set_onclick },
					_ => panic!("unknown event id"),
				};

				eprintln!("{}", listener.name.clone().unwrap());
				quote! {
					console::log_1(&#class.into());
					let element = document
						.get_elements_by_class_name(#class)
						.item(0)
						.expect("should never try to access a class with no members")
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
			})
			.collect()
	}
}

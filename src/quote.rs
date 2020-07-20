use {
	crate::tokens::*,
	syn::export::{quote::quote, TokenStream2},
};

pub struct List<'a> {
	id: &'a str,
	next: &'a Option<List<'a>>,
}

pub struct Context<'a> {
	pub path: Option<List<'a>>,
	pub r#type: Prefix,
}

pub trait Quote {
	fn quote(&self, context: &Context) -> TokenStream2;
}

impl Quote for Rule {
	fn quote(&self, context: &Context) -> TokenStream2 {
		let property = &self.property.to_string();
		let value = &self.value;
		let at_root = context.path.is_none();

		match &property.to_string()[..] {
			// meta information for the page and/or project must be defined at the top level
			"title" if at_root => {
				quote! {
					let element = meta.document.create_element("title").unwrap();
					meta.head.append_child(&element).unwrap();
					element.set_inner_html(#value);
				}
			}

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
					)?;
				}
			}
		}
	}
}

impl Quote for Block {
	fn quote(&self, context: &Context) -> TokenStream2 {
		let identifier = &self.identifier.to_string()[..];

		match self.prefix {
			Prefix::Instance => {
				let rule_quotes = self.rules.iter().map(|rule| rule.quote(context));
				let block_quotes = self.blocks.iter().map(|block| block.quote(context));

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

impl Quote for Document {
	fn quote(&self, context: &Context) -> TokenStream2 {
		let dom = self.root.quote(context);

		quote! {
			use {
				std::collections::HashMap,
				web_sys::{
					Document,
					Element,
					HtmlHeadElement,
					HtmlElement,
					Window,
				},
			};

			struct Meta {
				window: Window,
				document: Document,
				head: HtmlHeadElement,
				style: Element,
				// classes: HashMap<&'a str, Class<'a>>,
				// elements: HashMap<&'a str, &'a HtmlElement>,
			}

			struct Class<'a> {
				text: &'a str,
				styles: Vec<&'a str>,
			}
			impl Default for Class<'_> {
				fn default() -> Self {
					Class {
						text: "",
						styles: Vec::new(),
					}
				}
			}

			let window = web_sys::window().expect("getting window");
			let document = window.document().expect("getting `window.document`");
			let head = document.head().expect("getting `window.document.head`");
			let body = document.body().expect("getting `window.document.body`");
			let style = document.create_element("style").expect("creating a `style` element");
			head.append_child(&style).expect("appending `style` to `head`");
			let current_element = &body;
			let meta = Meta { window, document, head, style };
			#dom;
		}
	}
}

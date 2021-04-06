use std::convert::TryInto;

use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

impl Semantics {
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

			#[derive(Default)]
			struct Class {
				classes: HashMap<&'static str, Class>,
				listeners: Vec<Class>,
				elements: Vec<Class>,
				properties: HashMap<Property, &'static str>,
			}
		}
	}

	pub fn document(&self) -> TokenStream {
		let executable = self
			.pages
			.iter()
			.map(|page| self.static_element(page.root_id));

		quote! {
			let window = web_sys::window().expect("getting window");
			let document = &window.document().expect("getting `window.document`");
			let head = &document.head().expect("getting `window.document.head`");
			let body = &document.body().expect("getting `window.document.body`");
			let mut classes: HashMap<&'static str, Class> = HashMap::new();

			let element = body
				.children()
				.item(1) // item(0) is the <noscript> tag
				.expect("body should have a root element")
				.dyn_into::<HtmlElement>()
				.unwrap();
				#( #executable )*
		}
	}

	fn static_element(&self, element_id: usize) -> TokenStream {
		let classes = self.static_classes(element_id);
		let listeners = self.apply_listeners(element_id);
		let children = self.groups[element_id]
			.elements
			.iter()
			.enumerate()
			.map(|(i, child_id)| {
				let i: u32 = i.try_into().unwrap();
				let element = self.static_element(*child_id);
				quote! {
					{
						let element = element
							.children()
							.item(#i)
							.expect("aa")
							.dyn_into::<HtmlElement>()
							.unwrap();
						#element
					}
				}
			});
		quote! {
			#classes
			#listeners
			#( #children )*
		}
	}

	fn static_classes(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.classes
			.iter()
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				eprintln!("Class {} applies within {}", class_id, group_id);
				eprintln!("{:?}", self.groups[class_id]);
				let selector = self.groups[class_id]
					.selector
					.clone()
					.expect("static classes should have a selector");
				let rules = self.queue_all(class_id);
				quote! {
					{
						let class = classes
							.entry(#selector)
							.or_insert(Class::default());
						#rules
					}
				}
			})
			.collect()
	}
}

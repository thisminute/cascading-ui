use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote, std::convert::TryInto};

impl Semantics {
	pub fn header() -> TokenStream {
		let web_sys_includes = quote! {
			console,
			Document,
			Event,
			EventListener,
			HtmlElement,
		};

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
					#web_sys_includes
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
			struct Element {
				class_names: Vec<&'static str>,
				classes: HashMap<&'static str, Class>,
				listeners: Vec<Class>,
				elements: Vec<Element>,
				properties: HashMap<Property, &'static str>,
			}

			#[derive(Default)]
			struct Class {
				classes: HashMap<&'static str, Class>,
				listeners: Vec<Class>,
				elements: Vec<Element>,
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

			fn create_element(
				classes: &mut HashMap<&'static str, Class>,
				tag: &'static str,
				class_names: Vec<&'static str>,
			) -> HtmlElement {
				let window = web_sys::window().expect("getting window");
				let document = &window.document().expect("getting `window.document`");
				let mut element = document
					.create_element(tag)
					.expect(&*format!("Failed to create `{}` element.", tag))
					.dyn_into::<HtmlElement>()
					.unwrap();
				let mut queue = Vec::new();
				for class_name in class_names {
					if let Some(source) = classes.get(class_name) {
						for class in &source.classes {
							// let mut class = class.classes
							// 	.entry(#selector)
							// 	.or_insert(Class::default());
						}
						for class in &source.listeners {
						}
						for child in &source.elements {
							let tag = if child.properties.get(&Property::Link).is_some() {
								"a"
							} else {
								"div"
							};
							queue.push((tag, child.class_names.clone()));
						}
						for (property, value) in &source.properties {
							match property {
								Property::Css(property) => element.css(property, value),
								Property::Link => (),
								Property::Text => element.text(value),
								Property::Tooltip => (),
								Property::Image => (),
							}
						}
					}
				}

				for (tag, class_names) in queue {
					element
						.append_child(&create_element(classes, tag, class_names))
						.unwrap();
				}

				element
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
				.item(0)
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
				let child_id = *child_id;
				let i: u32 = i.try_into().unwrap();
				let element = self.static_element(child_id);
				quote! {
					{
						let element = element
							.children()
							.item(#i)
							.expect("should never try to index into an empty element")
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
						let mut class = classes
							.entry(#selector)
							.or_insert(Class::default());
						#rules
					}
				}
			})
			.collect()
	}
}

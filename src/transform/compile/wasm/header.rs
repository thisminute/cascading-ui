use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

impl Semantics {
	pub fn header() -> TokenStream {
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

				classes: HashMap<&'static str, Group>,
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

			fn create_element(
				tag: &'static str,
				class_names: Vec<&'static str>,
				classes: &HashMap<&'static str, Group>,
			) -> HtmlElement {
				let window = web_sys::window().expect("getting window");
				let document = &window.document().expect("getting `window.document`");
				let mut element = document
					.create_element(tag)
					.expect(&*format!("Failed to create `{}` element.", tag))
					.dyn_into::<HtmlElement>()
					.unwrap();

				let class_name = &*class_names.join(" ");
				element.set_class_name(class_name);

				let mut queue = Vec::new();
				for class_name in class_names {
					if let Some(source) = classes.get(class_name) {
						for class in &source.classes {
							// let mut class = class.classes
							// 	.entry(#selector)
							// 	.or_insert(Group::default());
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
						.append_child(&create_element(tag, class_names, classes))
						.unwrap();
				}

				element
			}

			thread_local! {
				static CLASSES: RefCell<HashMap<&'static str, Group>> = RefCell::new(HashMap::new());
		   }
		}
	}
}

use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

impl Semantics {
	pub fn runtime_functions() -> TokenStream {
		quote! {
			fn render_elements(
				group: &Group,
				parent: &mut HtmlElement,
				classes: &mut HashMap<&'static str, Group>,
			) {
				let window = web_sys::window().unwrap();
				let document = &window.document().unwrap();

				if !group.elements.is_empty() {
					while let Some(child) = parent.last_element_child() {
						parent.remove_child(&child.dyn_into::<Node>().unwrap()).unwrap();
					}
				}

				for element in &group.elements {
					let tag = if element.properties.get(&Property::Link).is_some() {
						"a"
					} else {
						"div"
					};
					let child = &mut document
						.create_element(tag)
						.unwrap()
						.dyn_into::<HtmlElement>()
						.unwrap();
					child.set_class_name(&element.class_names.join(" "));
					parent.append_child(child).unwrap();
					render_classes(element, classes);
					render_listeners(element, child);
					render_properties(element, child);
					render_elements(element, child, classes);
				}
			}

			fn render_classes(group: &Group, classes: &mut HashMap<&'static str, Group>) {
				let window = web_sys::window().unwrap();
				let document = &window.document().unwrap();

				for source in &group.classes {
					let mut target = classes.entry(source.selector).or_default();
					if !source.elements.is_empty() {
						target.elements = Vec::new();
						for element in &source.elements {
							target.elements.push(element.clone());
						}
					}
					for listener in &source.listeners {
						target.listeners.push(listener.clone());
					}
					for (property, value) in source.properties.clone() {
						target.properties.insert(property, value);
					}
				}

				for class in &group.classes {
					let elements = document.get_elements_by_class_name(class.selector);
					for i in 0..elements.length() {
						let element = &mut elements.item(i).unwrap().dyn_into::<HtmlElement>().unwrap();
						render_elements(class, element, classes);
						render_listeners(class, element);
						render_properties(class, element);
					}
					render_classes(class, classes);
				}
			}

			fn render_listeners(group: &Group, target: &mut HtmlElement) {
				for listener in &group.listeners {
					let closure = {
						let mut element = target.clone();
						let group = listener.clone();
						Closure::wrap(Box::new(move |e: Event| {
							e.stop_propagation();
							CLASSES.with(|classes| {
								let mut classes = classes.borrow_mut();

								// TODO: does this make sense if something else changes the group?
								render_classes(&group, &mut classes);
								render_elements(&group, &mut element, &mut classes);
								render_listeners(&group, &mut element);
								render_properties(&group, &mut element);
							});
						}) as Box<dyn FnMut(Event)>)
					};
					target.add_event_listener_with_callback(
							listener.selector,
							closure.as_ref().unchecked_ref(),
						)
						.unwrap();
					closure.forget();
				}
			}

			fn render_properties(group: &Group, element: &mut HtmlElement) {
				for (property, value) in &group.properties {
					render_property(element, property, value.clone());
				}
			}

			// fn render_variables(group: &Group) {
			// 	STATE.with(|state| {
			// 		let mut state = state.borrow_mut();
			// 		let window = web_sys::window().unwrap();
			// 		let document = &window.document().unwrap();
			// 		for (variable_id, value) in &group.variables {
			// 			render_variable(variable_id, value);
			// 		}
			// 	})
			// }

			// fn render_variable(id: usize, value: Value) {
			// 	state[*id] = value;
			// 	for (target, property, value) in effects[*id] {
			// 		match target {
			// 			EffectTarget::Element(element) => render_property(element, property, value),
			// 			EffectTarget::Class(class) => {
			// 				let elements = document.get_elements_by_class_name(class);
			// 				for i in 0..elements.length() {
			// 					let element = &mut elements
			// 						.item(i)
			// 						.unwrap()
			// 						.dyn_into::<HtmlElement>()
			// 						.unwrap();
			// 					render_property(element, property, value);
			// 				}
			// 			}
			// 			EffectTarget::Variable(variable_id) => render_variable(variable_id, value),
			// 		}
			// 	}
			// }

			fn render_property(element: &HtmlElement, property: &Property, value: Value) {
				match property {
					Property::Css(property) => element.css(property, value),
					Property::Link => (),
					Property::Text => element.text(value),
					Property::Tooltip => (),
					Property::Image => (),
				}
			}

			trait Std {
				fn text(&self, value: Value);
				fn css(&self, property: &'static str, value: Value);
			}

			impl Std for HtmlElement {
				fn text(&self, value: Value) {
					if let Some(element) = self
						.child_nodes()
						.item(0)
					{
						element.set_node_value(None);
					}
					if let Value::String(string) = value {
						self.prepend_with_str_1(string).unwrap();
					}
				}

				fn css(&self, property: &'static str, value: Value) {
					if let Value::String(string) = value {
						self.style().set_property(property, string).unwrap();
					}
				}
			}
		}
	}
}

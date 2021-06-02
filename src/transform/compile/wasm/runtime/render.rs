use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

impl Semantics {
	pub fn runtime_render_functions() -> TokenStream {
		quote! {
			fn render_elements(
				group: &Group,
				parent: &mut HtmlElement,
				classes: &mut HashMap<&'static str, Group>,
			) {
				let window = web_sys::window().unwrap();
				let document = &window.document().unwrap();
				for element in &group.elements {
					let tag = if element.properties.get(&Property::Link).is_some() {
						"a"
					} else {
						"div"
					};
					let mut child = &mut document
						.create_element(tag)
						.unwrap()
						.dyn_into::<HtmlElement>()
						.unwrap();
					child.set_class_name(&*element.class_names.join(" "));
					parent.append_child(&child).unwrap();
					render_classes(element, classes);
					render_listeners(element, child);
					render_properties(element, child);

					// TODO: verify if tail recursion is a thing
					render_elements(element, child, classes);
				}
			}

			fn render_classes(
				group: &Group,
				classes: &mut HashMap<&'static str, Group>,
			) {
				let window = web_sys::window().unwrap();
				let document = &window.document().unwrap();
				register_classes(group, classes);
				for class in &group.classes {
					let elements = document.get_elements_by_class_name(class.selector);
					for i in 0..elements.length() {
						let element = &mut elements
							.item(i)
							.unwrap()
							.dyn_into::<HtmlElement>()
							.unwrap();
						render_elements(class, element, classes);
						render_listeners(class, element);
						render_properties(class, element);
					}

					// TODO: verify if tail recursion is a thing
					render_classes(class, classes);
				}
			}

			fn render_listeners(
				group: &Group,
				target: &mut HtmlElement
			) {
				let window = web_sys::window().unwrap();
				let document = &window.document().unwrap();
				for listener in &group.listeners {
					let closure = {
						let mut element = target.clone();
						Closure::wrap(Box::new(move |e: Event| {
							// CLASSES.with(move |classes| {
							// 	e.stop_propagation();
							// 	let window = web_sys::window().unwrap();
							// 	let document = window.document().unwrap();
							// 	let mut classes = classes.borrow_mut();

							// 	// TODO: does this make sense if something else changes the group?
							// 	render_classes(group, &mut classes);
							// 	render_elements(group, &mut element, &mut classes);
							// 	render_listeners(group, &mut element);
							// 	render_properties(group, &mut element);
							// });
						}) as Box<dyn FnMut(Event)>)
					};
					target
						.add_event_listener_with_callback(
							listener.selector,
							closure.as_ref().unchecked_ref()
						)
						.unwrap();
					closure.forget();
				}
			}

			fn render_properties(group: &Group, element: &mut HtmlElement) {
				for (property, value) in &group.properties {
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
	}
}

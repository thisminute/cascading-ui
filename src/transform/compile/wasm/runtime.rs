use {
	data::semantics::{properties::CwlProperty, Semantics},
	proc_macro2::TokenStream,
	quote::quote,
	transform::compile::css::Css,
};

impl Semantics {
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
			let mut classes: HashMap<&'static str, Rule> = HashMap::new();

			#( #executable )*
		}
	}

	fn static_element(&self, element_id: usize) -> TokenStream {
		let classes = self.groups[element_id].classes.iter().map(|(_, groups)| {
			groups
				.iter()
				.map(|&group_id| self.class(group_id))
				.collect::<TokenStream>()
		});
		let listeners = self.groups[element_id]
			.listeners
			.iter()
			.map(|(class, listener_id)| self.listener(element_id, class, *listener_id));
		let children = self.groups[element_id]
			.elements
			.iter()
			.map(|&child_id| self.static_element(child_id));
		quote! {
			#( #classes )*
			#( #listeners )*
			#( #children )*
		}
	}

	// quote! {
	// 	document
	// 		.create_element(#tag)
	// 		.expect(&*format!("Failed to create `{}` element.", #tag))
	// 		.dyn_into::<HtmlElement>()
	// 		.unwrap();
	// };
	fn class(&self, class_id: usize) -> TokenStream {
		let class = &self.groups[class_id];
		// let classes = self.register_classes(target_id);
		// let elements = self.register_elements(target_id);
		let properties = class.properties.css.iter().map(|(property, value)| {
			let css = property.css();
			quote! {
				rule.properties
					.insert(Property::Css(#css), #value);
			}
		});

		let selector = class
			.selector
			.clone()
			.expect("classes should have a selector");
		quote! {
			let rule = classes.entry(#selector).or_insert(Rule {
				properties: HashMap::new(),
				elements: Vec::new(),
			});
			#( #properties )*
		}
	}

	fn listener(&self, target_id: usize, class: &String, listener_id: usize) -> TokenStream {
		let listener = &self.groups[listener_id];
		let properties = self.apply_properties(target_id);
		// let classes = self.register_classes(target_id);
		// let elements = self.register_elements(target_id);

		let event = match &*listener
			.name
			.clone()
			.expect("every listener should have an event id")
		{
			"click" => quote! { set_onclick },
			"mouseover" => quote! { set_onmouseover },
			_ => panic!("unknown event id"),
		};

		quote! {
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
					#properties
				}) as Box<dyn FnMut(Event)>)
			};
			element.#event(Some(on_click.as_ref().unchecked_ref()));
			on_click.forget();
		}
	}

	fn apply_properties(&self, group_id: usize) -> TokenStream {
		let group = &self.groups[group_id];

		let mut effects = Vec::new();
		if let Some(text) = group.properties.cwl.get(&CwlProperty::Text) {
			effects.push(quote! {
				element
					.child_nodes()
					.item(0)
					.unwrap()
					.set_node_value(None);
				element.prepend_with_str_1(#text).unwrap();
			});
		}
		if let Some(link) = group.properties.cwl.get(&CwlProperty::Link) {
			effects.push(quote! {
				document.location().unwrap().assign(#link).unwrap();
				element.style().set_property("cursor", "pointer").unwrap();
			});
		}

		for (property, value) in &group.properties.css {
			let property = property.css();
			effects.push(quote! {
				element.style().set_property(#property, #value).unwrap();
			});
		}

		quote! { #( #effects )* }
	}
}

mod compiled;
mod dynamic;
mod register;

use {crate::data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

// entry point for wasm compilation

impl Semantics {
	pub fn document(&self) -> TokenStream {
		let rules = self.compiled_element(self.pages[0].root_id);
		let router = self.compiled_router();

		if rules.is_empty() && router.is_empty() {
			return quote! {};
		}

		let classes_block = if rules.is_empty() {
			quote! {}
		} else {
			quote! {
				CLASSES.with(|classes| {
					let mut classes = classes.borrow_mut();
					let window = web_sys::window().expect("getting window");
					let document = &window.document().expect("getting `window.document`");
					let head = &document.head().expect("getting `window.document.head`");
					let body = &document.body().expect("getting `window.document.body`");

					let element = body
						.children()
						.item(0)
						.expect("body should have a root element")
						.dyn_into::<HtmlElement>()
						.unwrap();
					#rules
				});
			}
		};

		quote! {
			#classes_block
			#router
		}
	}

	/// Generate a hashchange-based router that shows/hides route elements.
	fn compiled_router(&self) -> TokenStream {
		if self.routes.is_empty() {
			return quote! {};
		}

		let route_names: Vec<&str> = self.routes.iter().map(|(name, _)| name.as_str()).collect();
		let default_route = &route_names[0];

		quote! {
			{
				let window = web_sys::window().unwrap();
				let document = window.document().unwrap();

				// Initial route from hash
				let hash = window.location().hash().unwrap_or_default();
				let initial_route = if hash.len() > 1 {
					&hash[1..]  // strip leading #
				} else {
					#default_route
				};

				// Apply initial route
				let route_names: Vec<&str> = vec![#( #route_names ),*];
				for route_name in &route_names {
					let selector = format!("[data-route='{}']", route_name);
					if let Some(el) = document.query_selector(&selector).unwrap() {
						let el = el.dyn_into::<HtmlElement>().unwrap();
						if *route_name == initial_route {
							el.style().set_property("display", "").unwrap();
						} else {
							el.style().set_property("display", "none").unwrap();
						}
					}
				}

				// Listen for hashchange
				let route_names_clone: Vec<String> = route_names.iter().map(|s| s.to_string()).collect();
				let closure = Closure::wrap(Box::new(move |_: Event| {
					let window = web_sys::window().unwrap();
					let document = window.document().unwrap();
					let hash = window.location().hash().unwrap_or_default();
					let target = if hash.len() > 1 {
						hash[1..].to_string()
					} else {
						route_names_clone[0].clone()
					};
					for route_name in &route_names_clone {
						let selector = format!("[data-route='{}']", route_name);
						if let Some(el) = document.query_selector(&selector).unwrap() {
							let el = el.dyn_into::<HtmlElement>().unwrap();
							if *route_name == target {
								el.style().set_property("display", "").unwrap();
							} else {
								el.style().set_property("display", "none").unwrap();
							}
						}
					}
				}) as Box<dyn FnMut(Event)>);
				window.add_event_listener_with_callback(
					"hashchange",
					closure.as_ref().unchecked_ref(),
				).unwrap();
				closure.forget();
			}
		}
	}
}

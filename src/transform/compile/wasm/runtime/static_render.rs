use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

impl Semantics {
	pub fn runtime_static_render_functions() -> TokenStream {
		quote! {
			fn static_render_element(
				tag: &'static str,
				class_names: Vec<&'static str>,
				classes: &mut HashMap<&'static str, Group>,
			) -> HtmlElement {
				let window = web_sys::window().unwrap();
				let document = &window.document().unwrap();
				let mut element = document
					.create_element(tag)
					.unwrap()
					.dyn_into::<HtmlElement>()
					.unwrap();
				element.set_class_name(&*class_names.join(" "));
				for class_name in class_names {
					if let Some(source) = classes.get(class_name) {
						// TODO: avoid cloning?
						let source = &source.clone();
						render_elements(source, &mut element, classes);
						render_listeners(source, &mut element);
						render_properties(source, &mut element);
					}
				}
				element
			}
		}
	}
}

mod compiled;
mod dynamic;
mod register;

use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

// entry point for wasm compilation

impl Semantics {
	pub fn document(&self) -> TokenStream {
		let rules = self.compiled_element(self.pages[0].root_id);
		if rules.is_empty() {
			return quote! {};
		}
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
	}
}

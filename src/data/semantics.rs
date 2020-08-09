use {
	super::dom::Element,
	std::collections::HashMap,
	syn::export::{quote::quote_spanned, Span, TokenStream2},
};

pub struct Class<'a> {
	pub text: &'a str,
	pub styles: Vec<&'a str>,
}
impl Default for Class<'_> {
	fn default() -> Self {
		Self {
			text: "",
			styles: Vec::new(),
		}
	}
}

pub struct Semantics<'a> {
	pub errors: Vec<TokenStream2>,
	pub warnings: Vec<TokenStream2>,

	pub title: Option<TokenStream2>,
	pub dom: Element,

	pub classes: HashMap<&'a str, Class<'a>>,
	pub elements: HashMap<String, &'a Element>,
}
impl<'a> Semantics<'a> {
	pub fn new() -> Self {
		Self {
			errors: Vec::new(),
			warnings: Vec::new(),

			title: None,
			dom: Element {
				children: Vec::new(),
			},

			classes: HashMap::new(),
			elements: HashMap::new(),
		}
	}

	// pub fn element(&'a mut self, context: &Context) -> &'a Element {
	// 	let mut parent = self.dom;
	// 	for i in context.path {
	// 		parent = parent.children[i];
	// 	}
	// 	&parent
	// }

	pub fn error(&mut self, message: &str) {
		self.errors.push(quote_spanned! {Span::call_site()=>
			compile_error!(#message);
		});
	}

	pub fn warning(&mut self, message: &str) {
		self.warnings.push(quote_spanned! {Span::call_site()=>
			compile_error!(#message);
		});
	}
}

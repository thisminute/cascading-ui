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
	pub dom: Option<Element<'a>>,

	pub classes: HashMap<&'a str, Class<'a>>,
	pub elements: HashMap<&'a str, &'a Element<'a>>,
}
impl Semantics<'_> {
	pub fn new() -> Self {
		Self {
			errors: Vec::new(),
			warnings: Vec::new(),

			title: None,
			dom: None,

			classes: HashMap::new(),
			elements: HashMap::new(),
		}
	}

	// pub fn get_element(&mut self, context: &Context) -> &Element {
	// 	self.elements[context.string]
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

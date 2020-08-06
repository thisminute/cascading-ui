use {
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
pub struct Meta<'a> {
	pub errors: Vec<TokenStream2>,
	pub warnings: Vec<TokenStream2>,
	pub title: Option<TokenStream2>,
	pub classes: HashMap<&'a str, Class<'a>>,
	pub elements: HashMap<&'a str, Class<'a>>,
}
impl Meta<'_> {
	pub fn new() -> Self {
		Self {
			errors: Vec::new(),
			warnings: Vec::new(),
			title: None,
			classes: HashMap::new(),
			elements: HashMap::new(),
		}
	}

	pub fn error(&mut self, span: Span, message: &str) {
		self.errors.push(quote_spanned! {span=>
			compile_error!(#message);
		});
	}

	pub fn warning(&mut self, span: Span, message: &str) {
		self.warnings.push(quote_spanned! {span=>
			compile_error!(#message);
		});
	}
}

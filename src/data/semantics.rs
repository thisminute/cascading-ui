use {
	super::{dom::Element, Context},
	std::{collections::HashMap, error::Error, fmt},
	syn::export::{quote::quote_spanned, Span, TokenStream2},
};

#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "There is an error: {}", self.0)
	}
}
impl Error for MyError {}

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
	pub only_header_wasm: bool,
	pub bindgen: bool,

	pub errors: Vec<TokenStream2>,
	pub warnings: Vec<TokenStream2>,

	pub title: Option<TokenStream2>,
	pub dom: Element<'a>,

	pub classes: HashMap<&'a str, Class<'a>>,
	pub elements: HashMap<&'a str, &'a Element<'a>>,
}
impl Semantics<'_> {
	pub fn new() -> Self {
		Self {
			only_header_wasm: false,

			errors: Vec::new(),
			warnings: Vec::new(),

			title: None,
			dom: Element::new(),

			classes: HashMap::new(),
			elements: HashMap::new(),

			bindgen: false,
		}
	}

	pub fn create_element_at_context(&mut self, context: &Context, size: usize) -> &Element {
		let mut current = &mut self.dom;
		for i in &context.path {
			current = &mut current.children[*i];
			current.active = true;
		}
		current.children.reserve_exact(size);
		for _ in 0..size {
			current.children.push(Element::new());
		}
		current
	}

	pub fn _get_element_by_context(&mut self, context: &Context) -> &Element {
		let mut current = &mut self.dom;
		for i in &context.path {
			current = &mut current.children[*i];
			current.active = true;
		}
		current
	}

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

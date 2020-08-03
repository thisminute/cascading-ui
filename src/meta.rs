use {std::collections::HashMap, syn::export::TokenStream2};
// pub struct List<'a> {
// 	id: &'a str,
// 	next: &'a Option<List<'a>>,
// }

pub struct Context {
	// 	pub path: Option<List<'a>>,
// pub r#type: Prefix,
}

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
	pub title: Option<TokenStream2>,
	pub classes: HashMap<&'a str, Class<'a>>,
}
impl Meta<'_> {
	pub fn new() -> Self {
		Self {
			errors: Vec::new(),
			title: None,
			classes: HashMap::new(),
		}
	}
}

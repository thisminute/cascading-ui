use {
	proc_macro2::TokenStream,
	quote::{quote, ToTokens},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum PageProperty {
	Title,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct CuiProperty(pub String);

impl ToTokens for CuiProperty {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match &*self.0 {
			"text" => quote! { Text },
			"link" => quote! { Link },
			"tooltip" => quote! { Tooltip },
			"image" => quote! { Image },
			name => panic!("invalid property {}", name),
		}
		.to_tokens(tokens)
	}
}

use {
	proc_macro2::TokenStream,
	quote::{quote, ToTokens},
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum PageProperty {
	Title,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum CuiProperty {
	Text,
	Link,
	Tooltip,
	Image,
}

impl ToTokens for CuiProperty {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Self::Text => quote! { Text },
			Self::Link => quote! { Link },
			Self::Tooltip => quote! { Tooltip },
			Self::Image => quote! { Image },
		}
		.to_tokens(tokens)
	}
}

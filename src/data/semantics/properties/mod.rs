mod css;
mod cui;

pub use self::cui::{CuiProperty, PageProperty};

use {
	self::css::CSS_PROPERTIES,
	super::Value,
	proc_macro2::TokenStream,
	quote::{quote, ToTokens},
	std::collections::HashMap,
};

pub type CssProperty = String;
pub type CssProperties = HashMap<CssProperty, Value>;
pub type CssRules = HashMap<String, CssProperties>;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Property {
	Css(String),
	Cui(CuiProperty),
	Page(PageProperty),
	Attribute(String),
}

pub type Properties = HashMap<Property, Value>;

fn is_css_property(name: &str) -> bool {
	CSS_PROPERTIES.contains(name)
}

impl Property {
	pub fn new(property: String) -> Self {
		if is_css_property(&property) {
			Self::Css(property)
		} else {
			match property.as_str() {
				"title" => Self::Page(PageProperty::Title),

				property => match property {
					"text" => Self::Cui(CuiProperty::Text),
					"link" => Self::Cui(CuiProperty::Link),
					"tooltip" => Self::Cui(CuiProperty::Tooltip),
					"image" => Self::Cui(CuiProperty::Image),
					"apply" => Self::Cui(CuiProperty::Apply),

					// Unrecognized properties become HTML attributes
					property => Self::Attribute(property.to_string()),
				},
			}
		}
	}
}

impl ToTokens for Property {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		quote! {Property::}.to_tokens(tokens);
		match self {
			Property::Css(name) => quote! { Css(#name) },
			Property::Cui(property) => match property {
				CuiProperty::Text => quote! { Text },
				CuiProperty::Link => quote! { Link },
				CuiProperty::Tooltip => quote! { Tooltip },
				CuiProperty::Image => quote! { Image },
				CuiProperty::Apply => quote! { Apply },
			},
			Property::Attribute(name) => quote! { Attribute(#name) },
			_ => quote! {},
		}
		.to_tokens(tokens)
	}
}

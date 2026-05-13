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
}

pub type Properties = HashMap<Property, Value>;

fn is_css_property(name: &str) -> bool {
	CSS_PROPERTIES.contains(name)
}

impl Property {
	pub fn new(property: String) -> Result<Self, String> {
		if is_css_property(&property) {
			Ok(Self::Css(property))
		} else {
			match property.as_str() {
				"title" => Ok(Self::Page(PageProperty::Title)),
				property => match property {
					"text" => Ok(Self::Cui(CuiProperty::Text)),
					"link" => Ok(Self::Cui(CuiProperty::Link)),
					"tooltip" => Ok(Self::Cui(CuiProperty::Tooltip)),
					"image" => Ok(Self::Cui(CuiProperty::Image)),
					"apply" => Ok(Self::Cui(CuiProperty::Apply)),
					property => Err(format!(
						"property '{}' is not recognized. Use a CSS property or a CUI property (text, link, tooltip, image, apply).",
						property
					)),
				},
			}
		}
	}
}

impl ToTokens for Property {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		quote! {Property::}.to_tokens(tokens);
		if let Property::Css(name) = self {
			quote! {Css(#name)}
		} else if let Property::Cui(property) = self {
			match property {
				CuiProperty::Text => quote! { Text },
				CuiProperty::Link => quote! { Link },
				CuiProperty::Tooltip => quote! { Tooltip },
				CuiProperty::Image => quote! { Image },
				CuiProperty::Apply => quote! { Apply },
			}
		} else {
			quote! {}
		}
		.to_tokens(tokens)
	}
}

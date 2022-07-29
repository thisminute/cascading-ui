mod css;
mod cui;
pub use self::cui::{CuiProperty, PageProperty};

use {self::css::CSS_PROPERTIES, super::Value, std::collections::HashMap};

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
	pub fn new(property: String) -> Self {
		if is_css_property(&property) {
			Self::Css(property)
		} else {
			match property.as_str() {
				"title" => Self::Page(PageProperty::Title),

				property => Self::Cui(match property {
					"text" => CuiProperty::Text,
					"link" => CuiProperty::Link,
					"tooltip" => CuiProperty::Tooltip,
					"image" => CuiProperty::Image,

					property => panic!(" property not recognized: {}", property),
				}),
			}
		}
	}
}

// use {quote::quote, TokenStream};
// 	match &**name {
// 		"blur" => quote! { set_onblur },
// 		"focus" => quote! { set_onfocus },
// 		"click" => quote! { set_onclick },
// 		"mouseover" => quote! { set_onmouseover },
// 		"mouseenter" => quote! { set_onmouseenter },
// 		"mouseleave" => quote! { set_onmouseleave },
// 		"mouseout" => quote! { set_onmouseout },
// 		_ => panic! {"unrecognized html event {}", name },
// 	}

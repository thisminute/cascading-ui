mod css;
mod cui;

pub use self::cui::{CuiProperty, PageProperty};
use data::ast::Value;
use std::collections::HashMap;
pub type CssProperty = String;

use self::css::CSS_PROPERTIES;

pub type PageProperties = HashMap<PageProperty, Value>;
pub type CssProperties = HashMap<CssProperty, Value>;
pub type CuiProperties = HashMap<CuiProperty, Value>;

pub type CssRules = HashMap<String, CssProperties>;

#[derive(Default, Clone)]
pub struct Properties {
	pub page: PageProperties,
	pub css: CssProperties,
	pub cui: CuiProperties,
}

pub fn is_css_property(name: &String) -> bool {
	CSS_PROPERTIES.contains(&&**name)
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

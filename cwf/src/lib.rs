extern crate proc_macro;
extern crate syn;

use {
	proc_macro::{
		TokenStream,
	},
	syn::{
		Expr,
		Ident,
		Token,
	},
	syn::{
		braced,
		export::{
			quote::quote,
			TokenStream2,
			ToTokens,
		},
		parse::{
			Parse,
			ParseStream,
		},
		parse_macro_input,
		token::Brace,
	},
};

#[derive(Debug)]
struct Rule {
	property: Ident,
	colon: Token![:],
	value: Expr,
	semicolon: Token![;],
}

impl Parse for Rule {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		Ok(Self {
			property: input.parse()?,
			colon: input.parse()?,
			value: input.parse()?,
			semicolon: input.parse()?,
		})
	}
}

impl ToTokens for Rule {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.property.to_tokens(tokens);
		self.value.to_tokens(tokens);
	}
}

#[derive(Debug)]
struct List {
	// prefix: Punct,
	identifier: Ident,
	brace: Brace,
	items: Items,
}

impl Parse for List {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let content;
		Ok(Self {
			// prefix: input.parse()?,
			identifier: input.parse()?,
			brace: braced!(content in input),
			items: content.parse()?,
		})
	}
}

impl ToTokens for List {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.identifier.to_tokens(tokens);
		self.items.to_tokens(tokens);
	}
}

#[derive(Debug)]
struct Items {
	rules: Vec<Rule>,
	lists: Vec<List>,
}

impl Parse for Items {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let mut rules = Vec::new();
		let mut lists = Vec::new();
		while input.peek(Ident) {
			while input.peek2(Token![:]) {
				rules.push(input.parse()?);
			}
			while input.peek2(Brace) {
				lists.push(input.parse()?);
			}
		}

		return Ok(Self {
			rules,
			lists,
		})
	}
}

impl ToTokens for Items {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for rule in &self.rules {
			rule.to_tokens(tokens);
		}
		for list in &self.lists {
			list.to_tokens(tokens);
		}
	}
}

#[proc_macro]
pub fn cwf(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as Items);
	let rules = ast.rules;
	// let lists = ast.lists;

	let init_scripts = rules.iter().map(
		|rule| {
			let property = &rule.property;
			let value = &rule.value;
			match &rule.property.to_string()[..] {
				"title" => {
					quote!{
						let element = &document.create_element("title")?;
						head.append_child(element)?;
						element.set_inner_html(#value);
					}
				},
				"content" => {
					quote!{
						let element = &document.create_element("div")?;
						body.append_child(element)?;
						element.set_inner_html(#value);
					}
				}
				_ => {

					quote!{
						current_element.style().set_property(
							&str::replace(stringify!(#property), "_", "-"),
							stringify!(#value)
						)?;
					}
				},
			}
		}
	);

	let expanded = quote!{
//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
use wasm_bindgen::prelude::*;

struct Meta<'a> {
	window: &'a web_sys::Window,
	document: &'a web_sys::Document,
	head: &'a web_sys::HtmlHeadElement,
	classes: std::collections::HashMap<&'a str, Class<'a>>,
	elements: std::collections::HashMap<&'a str, &'a web_sys::HtmlElement>,
}

struct Class<'a> {
	text: &'a str,
	styles: Vec<&'a str>,
}
impl Default for Class<'_> {
	fn default() -> Self { Class{
		text: "",
		styles: Vec::new(),
	} }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
	let window = &web_sys::window().expect("Failed to access global `window`.");
	let document = &window.document().expect("Failed to access `window.document`.");
	let head = &document.head().expect("Failed to access `window.document.head`.");
	let body = &document.body().expect("Failed to access `window.document.body`.");
	// let classes = std::collections::HashMap::new();
	let mut elements = std::collections::HashMap::new();
	elements.insert("body", body);
	// let meta = Meta { window, document, head, classes, elements };

	let mut current_element = elements.get("body").expect("Failed to get `body` element.");

	#(
		#init_scripts();
	)*

	// #(
	// 	let element = &document.create_element("div")?;
	// 	body.append_child(element)?;
	// 	element.set_inner_html(&format!("{}", stringify!(#rules))[..]);
	// 	// element.set_inner_html(&format!("{}", stringify!(#lists))[..]);
	// )*

	Ok(())
}
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
	};

	expanded.into()
}
// 	let name = &ast.identifier;
// 	let bname = format!("{}CWF", name);
// 	let bident = syn::Ident::new(&bname, name.span());
// 	let fields = if let syn::Data::Struct(syn::DataStruct {
// 		fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
// 		..
// 	}) = ast.data
// 	{
// 		named
// 	} else {
// 		unimplemented!();
// 	};

// 	let builder_fields = fields.iter().map(|f| {
// 		let name = &f.ident;
// 		let ty = &f.ty;
// 		if ty_inner_type("Option", ty).is_some() || builder_of(&f).is_some() {
// 			quote! { #name: #ty }
// 		} else {
// 			quote! { #name: std::option::Option<#ty> }
// 		}
// 	});

// 	let methods = fields.iter().map(|f| {
// 		let name = f.ident.as_ref().unwrap();
// 		let ty = &f.ty;

// 		let (arg_type, value) = if let Some(inner_ty) = ty_inner_type("Option", ty) {
// 			// if the field is an Option<T>, setting should take just a T,
// 			// but we then need to store it within a Some.
// 			(inner_ty, quote! { std::option::Option::Some(#name) })
// 		} else if builder_of(&f).is_some() {
// 			// if the field is a builder, it is a Vec<T>,
// 			// and the value in the builder is _not_ wrapped in an Option,
// 			// so we shouldn't wrap the value in Some.
// 			(ty, quote! { #name })
// 		} else {
// 			// otherwise, we take the type used by the target,
// 			// and we store it in an Option in the builder
// 			// in case it was never set.
// 			(ty, quote! { std::option::Option::Some(#name) })
// 		};
// 		let set_method = quote! {
// 			pub fn #name(&mut self, #name: #arg_type) -> &mut Self {
// 					self.#name = #value;
// 					self
// 			}
// 		};

// 		// we need to take care not to include a builder method with the same name as the set
// 		// method. for example, consider this struct:
// 		//
// 		// ```
// 		// #[derive(Builder)]
// 		// struct Command {
// 		//     #[builder(each = "env")]
// 		//     env: Vec<String>
// 		// }
// 		// ```
// 		//
// 		// It would not be okay to generate both `env(Vec<String>)` for the field
// 		// *and* `env(String)` for the builder.
// 		match extend_method(&f) {
// 			None => set_method.into(),
// 			Some((true, extend_method)) => extend_method,
// 			Some((false, extend_method)) => {
// 					// safe to generate both!
// 					let expr = quote! {
// 						#set_method
// 						#extend_method
// 					};
// 					expr.into()
// 			}
// 		}
// 	});

// 	// for when you call Builder::build
// 	let build_fields = fields.iter().map(|f| {
// 		let name = &f.ident;
// 		if ty_inner_type("Option", &f.ty).is_some() || builder_of(f).is_some() {
// 			quote! {
// 				#name: self.#name.clone()
// 			}
// 		} else {
// 			quote! {
// 				#name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
// 			}
// 		}
// 	});

// 	let build_empty = fields.iter().map(|f| {
// 		let name = &f.ident;
// 		if builder_of(f).is_some() {
// 			quote! { #name: std::vec::Vec::new() }
// 		} else {
// 			quote! { #name: std::option::Option::None }
// 		}
// 	});

// 	let doc = format!("\
// 		Implements the [builder pattern] for [`{}`].\n\
// 		\n\
// 		[builder pattern]: https://rust-lang-nursery.github.io/api-guidelines/type-safety.html#c-builder", name);

// 	let expanded = quote! {
// 		#[doc = #doc]
// 		pub struct #bident {
// 			#(#builder_fields,)*
// 		}
// 		impl #bident {
// 			#(#methods)*

// 			pub fn build(&self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
// 				std::result::Result::Ok(#name {
// 					#(#build_fields,)*
// 				})
// 			}
// 		}
// 		impl #name {
// 			pub fn builder() -> #bident {
// 				#bident {
// 					#(#build_empty,)*
// 				}
// 			}
// 		}
// 	};

// 	expanded.into()
// }

// fn builder_of(f: &syn::Field) -> Option<&syn::Attribute> {
// 	for attr in &f.attrs {
// 		if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
// 			return Some(attr);
// 		}
// 	}
// 	None
// }

// fn extend_method(f: &syn::Field) -> Option<(bool, proc_macro2::TokenStream)> {
// 	let name = f.ident.as_ref().unwrap();
// 	let g = builder_of(f)?;

// 	fn mk_err<T: quote::ToTokens>(t: T) -> Option<(bool, proc_macro2::TokenStream)> {
// 		Some((
// 			false,
// 			syn::Error::new_spanned(t, "expected `builder(each = \"...\")`").to_compile_error(),
// 		))
// 	}

// 	let meta = match g.parse_meta() {
// 		Ok(syn::Meta::List(mut nvs)) => {
// 			// list here is .. in #[builder(..)]
// 			assert_eq!(nvs.ident, "builder");
// 			if nvs.nested.len() != 1 {
// 				return mk_err(nvs);
// 			}

// 			// nvs.nested[0] here is (hopefully): each = "foo"
// 			match nvs.nested.pop().unwrap().into_value() {
// 				syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
// 					if nv.ident != "each" {
// 						return mk_err(nvs);
// 					}
// 					nv
// 				}
// 				meta => {
// 					// nvs.nested[0] was not k = v
// 					return mk_err(meta);
// 				}
// 			}
// 		}
// 		Ok(meta) => {
// 			// inside of #[] there was either just an identifier (`#[builder]`) or a key-value
// 			// mapping (`#[builder = "foo"]`), neither of which are okay.
// 			return mk_err(meta);
// 		}
// 		Err(e) => {
// 			return Some((false, e.to_compile_error()));
// 		}
// 	};

// 	match meta.lit {
// 		syn::Lit::Str(s) => {
// 			let arg = syn::Ident::new(&s.value(), s.span());
// 			let inner_ty = ty_inner_type("Vec", &f.ty).unwrap();
// 			let method = quote! {
// 				pub fn #arg(&mut self, #arg: #inner_ty) -> &mut Self {
// 					self.#name.push(#arg);
// 					self
// 				}
// 			};
// 			Some((&arg == name, method))
// 		}
// 		lit => panic!("expected string, found {:?}", lit),
// 	}
// }

// fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
// 	if let syn::Type::Path(ref p) = ty {
// 		if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
// 			return None;
// 		}

// 		if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
// 			if inner_ty.args.len() != 1 {
// 					return None;
// 			}

// 			let inner_ty = inner_ty.args.first().unwrap();
// 			if let syn::GenericArgument::Type(ref t) = inner_ty.value() {
// 					return Some(t);
// 			}
// 		}
// 	}
// 	None
// }

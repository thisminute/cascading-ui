extern crate proc_macro;
extern crate syn;
mod tokens;

use {
	std::fs::{
		read_dir,
		read_to_string,
	},
	crate::tokens::*,
	proc_macro::{
		TokenStream,
	},
	syn::{
		Expr,
		export::{
			quote::quote,
			TokenStream2,
			ToTokens,
		},
		parse_macro_input,
	},
};

fn rule_quote(rule: &Rule) -> TokenStream2 {
	let property = &rule.property.to_string();
	let value = &rule.value;
	match &property.to_string()[..] {
		"title" => {
			quote! {
				let element = &document.create_element("title").unwrap();
				head.append_child(element)?;
				element.set_inner_html(#value);
			}
		},
		"text" => {
			quote! {
				current_element.set_inner_html(#value);
			}
		},
		"href" => {
			match value {
				Expr::Lit(value) => {
					let value = value.into_token_stream().to_string();
					if value.contains(".") {
						if value.starts_with("http") {
							quote! {
								current_element.set_attribute("href", #value)?;
							}
						} else {
							quote! {
								current_element.set_attribute("href", &format!("https://{}", #value)[..])?;
							}
						}
					} else {
						quote! {
							let on_click = Closure::wrap(Box::new(|e: Event| {
								let element = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
								element.set_inner_html("hi");
							}) as Box<FnMut(Event)>);
							current_element.set_onclick(Some(on_click.as_ref().unchecked_ref()));
							on_click.forget();
						}
					}
				},
				_ => {
					quote! {}
				},
			}
		},
		"tip" => {
			quote! {
				current_element.set_attribute("title", #value)?;
			}
		},
		_ => {
			quote! {
				current_element.style().set_property(
					&str::replace(#property, "_", "-"),
					#value
				)?;
			}
		},
	}
}

fn list_quote(list: &List) -> TokenStream2 {
	let identifier = &list.identifier.to_string();
	let rule_quotes = list.rules.iter().map(rule_quote);
	let list_quotes = list.lists.iter().map(list_quote);
	let (descend, ascend) = if identifier != "_cwf" {
		(
			quote! {
				let element = &create_element(document, &#identifier);
				current_element.append_child(element).unwrap();
				let current_element = element;
			},
			quote! {
				let current_element = current_element.parent_element().unwrap();
			}
		)
	} else {
		(quote! {}, quote! {})
	};

	quote! {
		#descend

		#( #rule_quotes )*
		#( #list_quotes )*

		#ascend
	}
}

#[proc_macro]
pub fn cwf(input: TokenStream) -> TokenStream {
	// wrap the input in a list with the `cwf` identifier so that we can treate it as the root of a tree of lists
	let mut input = TokenStream2::from(input);

	// import .cwf files from the current directory and attach them to the input
	for entry in read_dir("./cwf").expect("should read the current directory") {
		match entry {
			Ok(entry) => {
				let filename = entry.path().display().to_string();
				if filename.ends_with(".cwf") {
					let contents: TokenStream2 = read_to_string(entry.path()).unwrap()[..].parse().unwrap();
					contents.to_tokens(&mut input);
				}
			},
			_ => {},
		}
	};
	let input = quote! {
		_cwf {
			#input
		}
	};

	eprintln!("input tokens: {}", input);

	// parse input into a struct
	let input = TokenStream::from(input);
	let list = &parse_macro_input!(input as List);

	// transform List object into Rust code that builds the dom
	let dom = list_quote(list);

	dom.into()
}

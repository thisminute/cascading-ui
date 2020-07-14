extern crate proc_macro;
extern crate syn;
mod tokens;

use {
	std::{
		fs::{
			read_dir,
			read_to_string,
		},
		path::Path,
	},
	crate::tokens::*,
	proc_macro::{
		TokenStream,
	},
	syn::{
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

fn list_quote(block: &Block) -> TokenStream2 {
	let identifier = &block.identifier.to_string();
	let rule_quotes = block.rules.iter().map(rule_quote);
	let block_quotes = block.blocks.iter().map(list_quote);
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
		#( #block_quotes )*

		#ascend
	}
}

#[proc_macro]
pub fn cwf(input: TokenStream) -> TokenStream {
	let mut input = TokenStream2::from(input);

	// if it exists, import .cwf files from the `cwf` directory and attach them to the input
	let path = "./cwf";
	if Path::new(path).exists() {
		for entry in read_dir(path).expect(&format!("reading from {}", path)[..]) {
			let entry = entry.expect("reading .cwf file");
			let filename = entry.path().display().to_string();
			if filename.ends_with(".cwf") {
				let contents: TokenStream2 = read_to_string(entry.path()).unwrap()[..].parse().unwrap();
				contents.to_tokens(&mut input);
			}
		}
	}

	// wrap the input in a list with a `_cwf` identifier so that we can treate it as the root of a tree of lists
	let input = quote! {
		_cwf {
			#input
		}
	};

	eprintln!("input tokens: {}", input);

	// parse input into a struct
	let input = TokenStream::from(input);
	let list = &parse_macro_input!(input as Block);

	eprintln!("Done parsing macro input");

	// transform Block object into Rust code that builds the dom
	let dom = list_quote(list);
	let expanded = quote! {
		extern crate wasm_bindgen;
		extern crate web_sys;
		use {
			wasm_bindgen::{
				prelude::*,
				JsCast,
			},
			web_sys::{
				Document,
				HtmlElement,
			},
		};

		// struct Meta<'a> {
		// 	window: &'a Window,
		// 	document: &'a Document,
		// 	head: &'a HtmlHeadElement,
		// 	classes: HashMap<&'a str, Class<'a>>,
		// 	elements: HashMap<&'a str, &'a HtmlElement>,
		// }

		// struct Class<'a> {
		// 	text: &'a str,
		// 	styles: Vec<&'a str>,
		// }
		// impl Default for Class<'_> {
		// 	fn default() -> Self { Class{
		// 		text: "",
		// 		styles: Vec::new(),
		// 	} }
		// }

		fn create_element(document: &Document, name: &str) -> HtmlElement {
			document
				.create_element(name)
				.expect(&format!("Failed to create `{}` element.", name)[..])
				.dyn_into::<HtmlElement>()
				.expect("Failed to construct element.")
		}

		#[wasm_bindgen(start)]
		pub fn run() -> Result<(), JsValue> {
			let window = &web_sys::window().expect("getting window");
			let document = &window.document().expect("getting `window.document`");
			let head = &document.head().expect("getting `window.document.head`");
			let body = &document.body().expect("getting `window.document.body`");
			let style = &document.create_element("style").expect("creating a `style` element");
			head.append_child(style).expect("appending `style` to `head`");
			// let classes = HashMap::new();
			// let mut elements = HashMap::new();
			// elements.insert("body", body).unwrap();
			let current_element = body;
			#dom;
			Ok(())
		}
	};

	expanded.into()
}

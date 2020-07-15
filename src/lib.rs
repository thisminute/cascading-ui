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
				head.append_child(element).unwrap();
				element.set_inner_html(#value);
			}
		}
		"text" => {
			quote! {
				current_element.set_inner_html("what");
			}
		}
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
		}
		"tip" => {
			quote! {
				current_element.set_attribute("title", #value)?;
			}
		}
		_ => {
			quote! {
				current_element.style().set_property(
					&str::replace(#property, "_", "-"),
					#value
				)?;
			}
		}
	}
}

fn block_quote(block: &Block) -> TokenStream2 {
	let identifier = &block.identifier.to_string()[..];
	let rule_quotes = block.rules.iter().map(rule_quote);
	let block_quotes = block.blocks.iter().map(block_quote);
	match block.prefix {
		Prefix::Instance => {}
		Prefix::Class => {}
		Prefix::Action => {}
		Prefix::Listener => {}
	}

	let quotes = quote! {
		#( #rule_quotes )*
		#( #block_quotes )*
	};

	match identifier {
		"_cwf" => { quotes }
		_ => {
			quote! {
				let element = &create_element(document, #identifier);
				current_element.append_child(element).unwrap();
				let current_element = element;

				#quotes

				let current_element = current_element.parent_element().unwrap();
			}
		}
	}
}

fn lib() -> TokenStream {
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

		fn create_element(document: &Document, name: &str) -> HtmlElement {
			document
				.create_element(name)
				.expect(&format!("Failed to create `{}` element.", name)[..])
				.dyn_into::<HtmlElement>()
				.expect("Failed to construct element.")
		}
	};
	expanded.into()
}

fn dom(input: TokenStream2) -> TokenStream {
	eprintln!("input tokens: {}", input);

	// wrap the input in a block with a `_cwf` identifier so that we can treate it as the root of a tree of blocks
	let input = quote! {
		_cwf {
			#input
		}
	};

	// parse input into a struct
	let input = TokenStream::from(input);
	let block = &parse_macro_input!(input as Block);

	// transform Block object into Rust code that builds the dom
	let dom = block_quote(block);

	let expanded = quote! {
		let window = &web_sys::window().expect("getting window");
		let document = &window.document().expect("getting `window.document`");
		let head = &document.head().expect("getting `window.document.head`");
		let body = &document.body().expect("getting `window.document.body`");
		let style = &document.create_element("style").expect("creating a `style` element");
		head.append_child(style).expect("appending `style` to `head`");
		let current_element = body;
		#dom;
	};

	expanded.into()
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

	let lib = TokenStream2::from(lib());
	let dom = TokenStream2::from(dom(input));
	let expanded = quote! {
		#lib

		#[wasm_bindgen(start)]
		pub fn run() -> Result<(), JsValue> {
			#dom;
			Ok(())
		}
	};

	expanded.into()
}

#[proc_macro]
pub fn cwf_dom(input: TokenStream) -> TokenStream {
	let input = TokenStream2::from(input);
	let expanded = dom(input);
	eprintln!("cwf_dom: *****************************\n {} \n **************************************", expanded);
	expanded.into()
}

#[proc_macro]
pub fn cwf_lib(_input: TokenStream) -> TokenStream {
	let expanded = lib();
	eprintln!("cwf_lib: {}", expanded);

	expanded.into()
}

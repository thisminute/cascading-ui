extern crate proc_macro;
extern crate syn;
mod parse;
mod quote;
mod to_tokens;
mod tokens;

use {
	crate::{quote::Quote, tokens::*},
	proc_macro::TokenStream,
	std::{
		fs::{read_dir, read_to_string, write},
		path::Path,
	},
	syn::{
		export::{quote::quote, ToTokens, TokenStream2},
		parse_macro_input,
	},
};

fn lib() -> TokenStream2 {
	quote! {
		extern crate wasm_bindgen;
		extern crate web_sys;
		use {
			wasm_bindgen::{
				prelude::*,
				JsCast,
			},
			web_sys::{
				Document,
				Event,
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
	}
}

fn dom(input: TokenStream) -> TokenStream {
	let dom = &parse_macro_input!(input as Document);
	dom.quote("").into()
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
				let contents: TokenStream2 =
					read_to_string(entry.path()).unwrap()[..].parse().unwrap();
				contents.to_tokens(&mut input);
			}
		}
	}

	let lib = TokenStream2::from(lib());
	let dom = TokenStream2::from(dom(input.into()));
	let expanded = quote! {
		#lib

		#[wasm_bindgen(start)]
		pub fn run() -> Result<(), JsValue> {
			#dom;
			Ok(())
		}
	};

	write("target/cwf_macro_output.rs", expanded.to_string()).unwrap();

	expanded.into()
}

#[proc_macro]
pub fn cwf_dom(input: TokenStream) -> TokenStream {
	dom(input).into()
}

#[proc_macro]
pub fn cwf_lib(_input: TokenStream) -> TokenStream {
	lib().into()
}

extern crate proc_macro;
extern crate syn;
mod lex;
mod meta;
mod parse;
mod quote;
mod to_tokens;
mod tokens;

use {
	crate::{
		lex::Lex,
		quote::Quote,
		tokens::{Document, Header, Website},
	},
	proc_macro::TokenStream,
	std::{
		fs::{read_dir, read_to_string, write},
		path::Path,
	},
	syn::{
		export::{ToTokens, TokenStream2},
		parse_macro_input,
	},
};

#[proc_macro]
pub fn cwl(input: TokenStream) -> TokenStream {
	let mut input = input.into();

	// if it exists, import .cwl files from the `cwl` directory and attach them to the input
	let path = "./cwl";
	if Path::new(path).exists() {
		for entry in read_dir(path).expect(&format!("reading from {}", path)[..]) {
			let entry = entry.expect("reading .cwl file");
			let filename = entry.path().display().to_string();
			if filename.ends_with(".cwl") {
				let contents: TokenStream2 = read_to_string(entry.path()).unwrap()[..].parse().unwrap();
				contents.to_tokens(&mut input);
			}
		}
	}

	let input = input.into();
	let mut document = parse_macro_input!(input as Document);
	document.lex();
	let expanded = Website { document }.quote();
	write("target/cwl_macro_output.rs", expanded.to_string()).unwrap();
	expanded.into()
}

#[proc_macro]
pub fn cwl_dom(input: TokenStream) -> TokenStream {
	let mut document = parse_macro_input!(input as Document);
	document.lex();
	document.quote().into()
}

#[proc_macro]
pub fn cwl_lib(_input: TokenStream) -> TokenStream {
	Header {}.quote().into()
}

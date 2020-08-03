extern crate html_minifier;
extern crate proc_macro;
extern crate syn;
mod html;
mod lex;
mod meta;
mod parse;
mod quote;
mod tokens;

use {
	crate::{
		html::Html,
		lex::Lex,
		meta::Meta,
		quote::Quote,
		tokens::{Document, Lib, Website},
	},
	html_minifier::HTMLMinifier,
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

/// all logic follows a chain of traits with one file per trait
/// each trait walks the AST recursively and never makes calls to the other traits
/// at the end of each trait execution, some new data structure is completely filled out for the next steps
/// not all traits
/// parse.rs -> tokens (see tokens.rs)
/// lex.rs   -> metadata (see meta.rs)
/// html.rs  -> target/cwl.html
/// quote.rs -> src/lib.rs
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
	let mut html_minifier = HTMLMinifier::new();
	let mut meta = Meta::new();

	// entry points for token traits
	let document = parse_macro_input!(input as Document);
	document.lex(&mut meta, None);
	document.html(&mut html_minifier);
	let expanded = Website { document }.quote(&meta, None);

	write("target/cwl.html", html_minifier.get_html()).unwrap();

	write("target/cwl_macro_output.rs", expanded.to_string()).unwrap();

	expanded.into()
}

#[proc_macro]
pub fn cwl_dom(input: TokenStream) -> TokenStream {
	let input = input.into();
	let mut html_minifier = HTMLMinifier::new();
	let mut meta = Meta::new();

	// entry points for token traits
	let document = parse_macro_input!(input as Document);
	document.lex(&mut meta, None);
	document.html(&mut html_minifier);
	document.quote(&meta, None).into()
}

#[proc_macro]
pub fn cwl_lib(_input: TokenStream) -> TokenStream {
	Lib {}.quote(&Meta::new(), None).into()
}

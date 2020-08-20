extern crate html_minifier;
extern crate proc_macro;
extern crate syn;
mod data;
mod misc;
mod transform;

use {
	data::{ast::Document, Semantics},
	html_minifier::HTMLMinifier,
	proc_macro::TokenStream,
	std::{
		error::Error,
		fs::{read_dir, read_to_string, write},
		path::Path,
	},
	syn::{
		export::{ToTokens, TokenStream2},
		parse_macro_input,
	},
	transform::{
		semantic_analysis,
		write::{Html, Wasm},
	},
};

type BoxResult<T> = Result<T, Box<dyn Error>>;

fn pipeline(document: Document, bindgen_start: bool) -> (HTMLMinifier, TokenStream2) {
	let mut semantics = Semantics::new();
	semantic_analysis(&document, &mut semantics);
	if bindgen_start {
		semantics.bindgen = true;
	}

	let mut html_minifier = HTMLMinifier::new();
	semantics.html(&mut html_minifier).unwrap();
	let wasm = semantics.wasm();

	(html_minifier, wasm)
}

/// all logic follows a chain of traits with one file per trait
/// each trait walks the AST recursively and never makes calls to the other traits
/// at the end of each trait execution, some new data structure is completely filled out for the next steps
/// not all traits
/// parse.rs -> tokens (see tokens.rs)
/// lex.rs   -> metadata (see semantics.rs)
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
	let (mut index, runtime) = pipeline(parse_macro_input!(input as Document), true);

	write("target/index.html", index.get_html()).unwrap();
	write("target/cwl_macro_output.rs", runtime.to_string()).unwrap();

	runtime.into()
}

#[proc_macro]
pub fn cwl_dom(input: TokenStream) -> TokenStream {
	let (_index, runtime) = pipeline(parse_macro_input!(input as Document), true);
	runtime.into()
}

#[proc_macro]
pub fn cwl_lib(_input: TokenStream) -> TokenStream {
	let mut semantics = Semantics::new();
	semantics.only_header_wasm = true;
	semantics.wasm().into()
}

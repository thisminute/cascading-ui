extern crate html_minifier;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;
mod data;
mod misc;
mod transform;

use {
	data::{ast::Document, dom::Dom, Semantics},
	proc_macro::TokenStream,
	proc_macro2::TokenStream as TokenStream2,
	quote::ToTokens,
	std::{
		fs::{read_dir, read_to_string, write},
		path::Path,
	},
	syn::parse_macro_input,
	transform::write::Wasm,
};

// type BoxResult<T> = Result<T, Box<dyn std::error::ErrorError>>;

fn pipeline(document: Document, bindgen_start: bool) -> (Vec<(String, String)>, TokenStream2) {
	let mut semantics = document.analyze(bindgen_start);
	let dom = semantics.render();
	(dom.html(), dom.wasm(&semantics))
}

#[proc_macro]
pub fn cwl(input: TokenStream) -> TokenStream {
	let mut input = input.into();

	// if it exists, import .cwl files from the `cwl` directory and attach them to the input
	let path = "./cwl";
	if Path::new(path).exists() {
		for entry in read_dir(path).expect(&*format!("reading from {}", path)) {
			let entry = entry.expect("reading .cwl file");
			let filename = entry.path().display().to_string();
			if filename.ends_with(".cwl") {
				let contents: TokenStream2 = read_to_string(entry.path()).unwrap().parse().unwrap();
				contents.to_tokens(&mut input);
			}
		}
	}

	let input = input.into();
	let (pages, runtime) = pipeline(parse_macro_input!(input as Document), true);

	for (filename, html) in pages {
		let destination = format!("target/html/{}", filename);
		write(&destination, html).expect(&*format!("writing output html code to {}", destination));
	}
	write("target/cwl_macro_output.rs", runtime.to_string()).expect("writing output rust code");

	runtime.into()
}

#[proc_macro]
pub fn cwl_document(input: TokenStream) -> TokenStream {
	let (_pages, runtime) = pipeline(parse_macro_input!(input as Document), false);
	runtime.into()
}

#[proc_macro]
pub fn cwl_header(_input: TokenStream) -> TokenStream {
	let mut semantics = Semantics::new(false);
	let dom = Dom::new();
	semantics.only_header_wasm = true;
	dom.wasm(&semantics).into()
}

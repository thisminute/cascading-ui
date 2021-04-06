use {
	data::ast::{Block, Document, Prefix, Property},
	proc_macro2::Span,
	syn::{braced, ext::IdentExt, parse::ParseStream, token::Brace, Ident, Token},
};

fn peek_property(input: ParseStream) -> bool {
	input.peek(Ident::peek_any) && input.peek2(Token![:])
}
fn peek_block(input: ParseStream) -> bool {
	input.peek(Ident::peek_any) && input.peek2(Brace)
}
fn peek_class_block(input: ParseStream) -> bool {
	input.peek(Token![.]) && input.peek2(Ident::peek_any) && input.peek3(Brace)
}
fn peek_listener_block(input: ParseStream) -> bool {
	input.peek(Token![?]) && input.peek2(Ident::peek_any) && input.peek3(Brace)
}

// impl Parse for HyphenatedIdent {
// 	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
// 		let mut parts = Vec::new();
// 		while input.peek(Ident::peek_any) {
// 			parts.push(input.parse()?);
// 			match input.parse::<Token![-]>() {
// 				Ok(_) => {
// 					continue;
// 				}
// 				Err(_) => {
// 					break;
// 				}
// 			}
// 		}

// 		Ok(Self { parts })
// 	}
// }

pub use syn::parse::Parse;

impl Parse for Document {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		Ok(Self {
			root: parse_content(input, Ident::new("_", Span::call_site()), Prefix::Element)?,
		})
	}
}

fn parse_content(
	input: ParseStream,
	identifier: Ident,
	prefix: Prefix,
) -> Result<Block, syn::Error> {
	let mut block = Block {
		identifier,
		prefix,
		properties: Vec::new(),
		elements: Vec::new(),
		classes: Vec::new(),
		listeners: Vec::new(),
	};
	loop {
		if peek_property(input) {
			block.properties.push(input.parse()?);
		} else if peek_block(&input) {
			block.elements.push(input.parse()?);
		} else if peek_class_block(&input) {
			block.classes.push(input.parse()?);
		} else if peek_listener_block(&input) {
			block.listeners.push(input.parse()?);
		} else {
			break;
		}
	}
	Ok(block)
}

impl Parse for Block {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let prefix = if input.peek(Token![.]) {
			input.parse::<Token![.]>()?;
			Prefix::Class
		} else if input.peek(Token![?]) {
			input.parse::<Token![?]>()?;
			Prefix::Listener
		} else {
			Prefix::Element
		};

		let identifier = input.parse()?;

		let content;
		braced!(content in input);

		parse_content(&content, identifier, prefix)
	}
}

impl Parse for Property {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let property = input.parse()?;
		input.parse::<Token![:]>()?;
		let value = input.parse()?;
		input.parse::<Token![;]>()?;
		Ok(Self { property, value })
	}
}

extern crate proc_macro;
extern crate syn;

use syn::{
	braced,
	export::{ToTokens, TokenStream2},
	ext::IdentExt,
	parse::{Parse, ParseStream},
	token::Brace,
	Expr, Ident, Token,
};

#[derive(Debug)]
pub struct HyphenatedIdent {
	pub parts: Vec<Ident>,
}

impl Parse for HyphenatedIdent {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		eprintln!("HyphenatedIdent");
		let mut parts = Vec::new();
		while input.peek(Ident::peek_any) {
			parts.push(input.parse()?);
			match input.parse::<Token![-]>() {
				Ok(_) => {
					continue;
				}
				Err(_) => {
					break;
				}
			}
		}

		Ok(Self { parts })
	}
}

impl ToTokens for HyphenatedIdent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.to_string().to_tokens(tokens)
	}
}

impl ToString for HyphenatedIdent {
	fn to_string(&self) -> String {
		let result = self
			.parts
			.iter()
			.map(|ident| ident.to_string())
			.collect::<Vec<String>>()
			.join("-");
		result
	}
}

#[derive(Debug)]
pub struct Rule {
	pub property: Ident,
	pub value: Expr,
}

impl Parse for Rule {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		eprintln!("RuleParse");
		let property = input.parse()?;
		input.parse::<Token![:]>()?;
		let value = input.parse()?;
		input.parse::<Token![;]>()?;
		eprintln!("Done RuleParse");
		Ok(Self { property, value })
	}
}

impl ToTokens for Rule {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.property.to_tokens(tokens);
		self.value.to_tokens(tokens);
	}
}

#[derive(Debug)]
pub enum Prefix {
	Instance,
	Class,
	Action,
	Listener,
}

#[derive(Debug)]
pub struct Block {
	pub prefix: Prefix,
	pub identifier: Ident,
	pub rules: Vec<Rule>,
	pub blocks: Vec<Block>,
}

// can't implement Peek for some reason, so have to use these silly functions instead
fn peek_rule(input: ParseStream) -> bool {
	input.peek(Ident::peek_any) && input.peek2(Token![:])
}
fn peek_block(input: ParseStream) -> bool {
	input.peek(Ident::peek_any) && input.peek2(Brace)
}
fn peek_prefixed_block(input: ParseStream) -> bool {
	(input.peek(Token![.]) || input.peek(Token![!]) || input.peek(Token![?]))
		&& input.peek2(Ident::peek_any)
		&& input.peek3(Brace)
}

impl Parse for Block {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		eprintln!("BlockParse");

		let prefix = if input.peek(Token![.]) {
			input.parse::<Token![.]>()?;
			Prefix::Class
		} else if input.peek(Token![!]) {
			input.parse::<Token![!]>()?;
			Prefix::Action
		} else if input.peek(Token![?]) {
			input.parse::<Token![?]>()?;
			Prefix::Listener
		} else {
			Prefix::Instance
		};

		let identifier = input.parse()?;
		eprintln!("identifier: {}", identifier);

		let content;
		braced!(content in input);

		let mut rules = Vec::new();
		let mut blocks = Vec::new();
		loop {
			if peek_rule(&content) {
				rules.push(content.parse()?);
			} else if peek_block(&content) || peek_prefixed_block(&content) {
				blocks.push(content.parse()?);
			} else {
				break;
			}
		}

		Ok(Self {
			prefix,
			identifier,
			rules,
			blocks,
		})
	}
}

impl ToTokens for Block {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.identifier.to_tokens(tokens);
		for rule in &self.rules {
			rule.to_tokens(tokens);
		}
		for block in &self.blocks {
			block.to_tokens(tokens);
		}
	}
}

extern crate proc_macro;
extern crate syn;

use {
	syn::{
		Expr,
		Ident,
		Token,
	},
	syn::{
		braced,
		export::{
			TokenStream2,
			ToTokens,
		},
		parse::{
			Parse,
			ParseStream,
		},
		token::Brace,
	},
};

#[derive(Debug)]
pub struct CwfIdent {
	pub parts: Vec<Ident>,
}

impl Parse for CwfIdent {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		eprint!("CwfIdentParse ");

		let mut parts = Vec::new();
		while input.peek(Ident) {
			parts.push(input.parse()?);
			// let thing2: Token![-] = input.parse()?;
		}

		Ok(Self {
			parts,
		})
	}
}

impl ToTokens for CwfIdent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for part in &self.parts {
			part.to_tokens(tokens);
		}
	}
}

impl ToString for CwfIdent {
	fn to_string(&self) -> String {
		self.parts
			.iter()
			.map(|ident| { ident.to_string() })
			.collect::<String>()
	}
}

#[derive(Debug)]
pub struct Rule {
	pub property: Ident,
	pub value: Expr,
}

impl Parse for Rule {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		eprint!("RuleParse ");
		let property = input.parse()?;
		input.parse::<Token![:]>()?;
		let value = input.parse()?;
		input.parse::<Token![;]>()?;
		Ok(Self {
			property,
			value,
		})
	}
}

impl ToTokens for Rule {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.property.to_tokens(tokens);
		self.value.to_tokens(tokens);
	}
}

#[derive(Debug)]
pub struct List {
	// prefix: Punct,
	pub identifier: Ident,
	pub rules: Vec<Rule>,
	pub lists: Vec<List>,
}

impl Parse for List {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		eprint!("ListParse ");
		let identifier = input.parse()?;

		let content;
		braced!(content in input);

		let mut rules = Vec::new();
		let mut lists = Vec::new();
		while content.peek(Ident) {
			if content.peek2(Token![:]) {
				rules.push(content.parse()?);
			} else if content.peek2(Brace) {
				lists.push(content.parse()?);
			} else {
				break;
			}
		}

		Ok(Self {
			identifier,
			rules,
			lists,
		})
	}
}

impl ToTokens for List {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.identifier.to_tokens(tokens);
		for rule in &self.rules {
			rule.to_tokens(tokens);
		}
		for list in &self.lists {
			list.to_tokens(tokens);
		}
	}
}

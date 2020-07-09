extern crate proc_macro;
extern crate syn;

use {
	syn::{
		Expr,
		ext::IdentExt,
		Ident,
		Token,

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
				Ok(_) => { continue; },
				Err(_) => { break; },
			}
		}

		Ok(Self {
			parts,
		})
	}
}

impl ToTokens for HyphenatedIdent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.to_string().to_tokens(tokens)
	}
}

impl ToString for HyphenatedIdent {
	fn to_string(&self) -> String {
		let result = self.parts
			.iter()
			.map(|ident| { ident.to_string() })
			.collect::<Vec<String>>()
			.join("-");
		result
	}
}


// #[derive(Debug)]
// pub struct Item {
// 	pub is_rule: bool,
// 	pub rule: Rule,
// 	pub list: List,
// }

// impl Parse for Item {
// 	fn parse(input: ParseStream) -> Result<Self, syn::Error> {

// 	}
// }

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
		eprintln!("ListParse");
		let identifier = input.parse()?;
		eprintln!("{}", identifier);

		let content;
		braced!(content in input);

		let mut rules = Vec::new();
		let mut lists = Vec::new();
		while content.peek(Ident::peek_any) {
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

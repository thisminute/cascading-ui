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
pub struct Rule {
	pub property: Ident,
	colon: Token![:],
	pub value: Expr,
	semicolon: Token![;],
}

impl Parse for Rule {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		Ok(Self {
			property: input.parse()?,
			colon: input.parse()?,
			value: input.parse()?,
			semicolon: input.parse()?,
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
            content.error("Unexpected identifier.");
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

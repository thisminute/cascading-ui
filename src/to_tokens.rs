use {
	crate::tokens::*,
	syn::export::{ToTokens, TokenStream2},
};

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

impl ToTokens for HyphenatedIdent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.to_string().to_tokens(tokens)
	}
}

impl ToTokens for Rule {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.property.to_tokens(tokens);
		self.value.to_tokens(tokens);
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

use syn::{ext::IdentExt, parse::ParseStream, token::Brace, Ident, Token};

pub trait Peek {
	fn peek_property(&self) -> bool;
	fn peek_element_block(&self) -> bool;
	fn peek_class_block(&self) -> bool;
	fn peek_listener_block(&self) -> bool;
	fn peek_variable(&self) -> bool;
	fn peek_assignment(&self) -> bool;

	// keywords
	fn peek_use(&self) -> bool;
}

impl Peek for ParseStream<'_> {
	fn peek_property(&self) -> bool {
		self.peek(Ident::peek_any) && self.peek2(Token![:])
	}
	fn peek_element_block(&self) -> bool {
		(self.peek(Ident::peek_any) || self.peek(Token![_])) && self.peek2(Brace)
	}
	fn peek_class_block(&self) -> bool {
		self.peek(Token![.]) && self.peek2(Ident::peek_any) && self.peek3(Brace)
	}
	fn peek_listener_block(&self) -> bool {
		self.peek(Token![?]) && self.peek2(Ident::peek_any) && self.peek3(Brace)
	}
	fn peek_assignment(&self) -> bool {
		self.peek_variable() && self.peek3(Token![:])
	}
	fn peek_variable(&self) -> bool {
		self.peek(Token![$]) && self.peek2(Ident::peek_any)
	}

	fn peek_use(&self) -> bool {
		self.peek(Token![use]) && self.peek2(Token![:])
	}
}

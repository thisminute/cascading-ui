use crate::{
	context::{Ancestor, Context, Info},
	meta::Meta,
	syn::{export::quote::quote, spanned::Spanned},
	tokens::*,
};

pub trait Lex {
	fn lex(&self, meta: &mut Meta, context: &mut Context);
}

impl Lex for Document {
	fn lex(&self, meta: &mut Meta, context: &mut Context) {
		self.root.lex(meta, context);

		match &meta.title {
			Some(_) => {}
			None => meta.warning(
				self.root.identifier.span(),
				"you must set a title for the page",
			),
		}
	}
}

impl Lex for Block {
	fn lex(&self, meta: &mut Meta, context: &mut Context) {
		context.push(Ancestor {
			r#type: self.prefix,
			string: self.identifier.to_string(),
		});
		match self.prefix {
			Prefix::Instance => {
				for rule in &self.rules {
					rule.lex(meta, context);
				}
			}
			Prefix::Class => {}
			Prefix::Action => {}
			Prefix::Listener => {}
		};
		context.pop();
	}
}

impl Lex for Rule {
	fn lex(&self, meta: &mut Meta, context: &mut Context) {
		let property = self.property.to_string();
		let value = &self.value;
		let at_root = context.is_root();

		match &property.to_string()[..] {
			// meta information for the page and/or project must be defined at the top level
			"title" if at_root => match &meta.title {
				Some(_title) => meta.error(value.span(), "title property cannot be set more than once"),
				None => meta.title = Some(quote! { #value }),
			},

			"text" => {}
			"link" => {}
			"tip" => {}
			_ => {}
		}
	}
}

use {
	crate::{
		meta::{Context, Meta},
		tokens::*,
	},
	syn::{
		export::quote::{quote, quote_spanned},
		spanned::Spanned,
	},
};

pub trait Lex {
	fn lex(&mut self);
}

trait ContextLex {
	fn lex(&self, meta: &mut Meta, context: Option<&Context>);
}

impl Lex for Document<'_> {
	fn lex(&mut self) {
		self.root.lex(&mut self.meta, None);
	}
}

impl ContextLex for Block {
	fn lex(&self, meta: &mut Meta, context: Option<&Context>) {
		// let identifier = &self.identifier.to_string()[..];

		match self.prefix {
			Prefix::Instance => {
				for rule in &self.rules {
					rule.lex(meta, context);
				}

				// let block_quotes = self.blocks.iter().map(|block| block.lex(document, context));

				// match identifier {
				// 	_ => {
				// 		quote! {
				// 			let element = &create_element(meta.document, #identifier);
				// 			current_element.append_child(element).unwrap();
				// 			let current_element = element;

				// 			#quotes

				// 			let current_element = current_element.parent_element().unwrap();
				// 		}
				// 	}
				// }
			}
			Prefix::Class => {}
			Prefix::Action => {}
			Prefix::Listener => {}
		};
	}
}

impl ContextLex for Rule {
	fn lex(&self, meta: &mut Meta, _context: Option<&Context>) {
		let property = self.property.to_string();
		let value = &self.value;
		let span = self.value.span();
		let at_root = true; // context.path.is_none();

		match &property.to_string()[..] {
			// meta information for the page and/or project must be defined at the top level
			"title" if at_root => {
				meta.title = Some(match &meta.title {
					Some(_title) => quote_spanned! {
						span=>
						compile_error!("title property cannot be set more than once")
					},
					None => {
						quote! { #value }
					}
				})
			}

			"text" => {}
			"link" => {}
			"tip" => {}
			_ => {}
		}
	}
}

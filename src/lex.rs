use crate::{
	meta::{Context, Meta},
	tokens::*,
};

pub trait Lex {
	fn lex(&self, context: Option<&Context>, meta: Option<&mut Meta>);
}

impl Lex for Cwl {
	fn lex(&self, context: Option<&Context>, meta: Option<&mut Meta>) {
		self.root.lex(context)
	}
}

impl Lex for Document {
	fn lex(&self, context: Option<&Context>, meta: Option<&mut Meta>) {
		self.root.lex(context)
	}
}

impl Lex for Rule {
	fn lex(&self, context: Option<&Context>, meta: Option<&mut Meta>) {
		let property = self.property.to_string();
		let value = self.value;
		let at_root = context.path.is_none();

		match &property.to_string()[..] {
			// meta information for the page and/or project must be defined at the top level
			"title" if at_root => match meta.title {
				Some(title) => compile_error!("aaaa"),
				None => meta.title,
			},

			"text" => {}
			"link" => {}
			"tip" => {}
			_ => {}
		}
	}
}

impl Lex for Block {
	fn lex(&self, context: Option<&Context>, meta: Option<&mut Meta>) {
		let identifier = self.identifier.to_string()[..];

		match self.prefix {
			Prefix::Instance => {
				let rule_quotes = self.rules.iter().map(|rule| rule.quote(context));
				let block_quotes = self.blocks.iter().map(|block| block.quote(context));

				let quotes = quote! {
					#( #rule_quotes )*
					#( #block_quotes )*
				};

				match identifier {
					_ => {
						quote! {
							let element = &create_element(meta.document, #identifier);
							current_element.append_child(element).unwrap();
							let current_element = element;

							#quotes

							let current_element = current_element.parent_element().unwrap();
						}
					}
				}
			}
			Prefix::Class => {
				quote! {}
			}
			Prefix::Action => {
				quote! {}
			}
			Prefix::Listener => {
				quote! {}
			}
		}
	}
}

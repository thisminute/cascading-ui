mod peek;

use {
	self::peek::Peek,
	crate::data::ast::{
		Assignment, Block, Document, KeyframeStep, KeyframesBlock, Prefix, Property, Value,
		Variable,
	},
	proc_macro2::Span,
	syn::{
		braced,
		ext::IdentExt,
		parse::{Parse, ParseStream},
		Ident, LitInt, LitStr, Token,
	},
};

impl Parse for Document {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let root = parse_content(input, Ident::new("_", Span::call_site()), Prefix::Element)?;
		let keyframes = root.keyframes.clone();
		Ok(Self { root, keyframes })
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
		keyframes: Vec::new(),
		variables: Vec::new(),
		assignments: Vec::new(),
	};
	loop {
		if input.peek_declaration() {
			input.parse::<Token![let]>()?;
			let assignment = input.parse::<Assignment>()?;
			block
				.variables
				.push((assignment.variable.0.to_string(), assignment.value));
		} else if input.peek_property() {
			block.properties.push(input.parse()?);
		} else if input.peek_element_block() {
			block.elements.push(input.parse()?);
		} else if input.peek_class_block() {
			block.classes.push(input.parse()?);
		} else if input.peek_listener_block() {
			block.listeners.push(input.parse()?);
		} else if input.peek_assignment() {
			let assignment = input.parse::<Assignment>()?;
			block
				.assignments
				.push((assignment.variable.0.to_string(), assignment.value));
		} else if input.peek(Token![@]) {
			block.keyframes.push(parse_keyframes(input)?);
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

fn parse_keyframes(input: ParseStream) -> Result<KeyframesBlock, syn::Error> {
	input.parse::<Token![@]>()?;
	let directive: Ident = input.call(Ident::parse_any)?;
	if directive != "keyframes" {
		return Err(syn::Error::new(
			directive.span(),
			format!("expected 'keyframes' after '@', got '{}'", directive),
		));
	}
	let name: Ident = input.call(Ident::parse_any)?;

	let content;
	braced!(content in input);

	let mut steps = Vec::new();
	while !content.is_empty() {
		let selector: Ident = content.call(Ident::parse_any)?;
		let step_content;
		braced!(step_content in content);

		let mut properties = Vec::new();
		while !step_content.is_empty() {
			properties.push(step_content.parse::<Property>()?);
		}

		steps.push(KeyframeStep {
			selector: selector.to_string(),
			properties,
		});
	}

	Ok(KeyframesBlock {
		name: name.to_string(),
		steps,
	})
}

impl Parse for Property {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		// Parse hyphenated property names like font-family, max-width, etc.
		let mut property = input.parse::<Ident>()?.to_string();
		while input.peek(Token![-]) {
			input.parse::<Token![-]>()?;
			let next = input.parse::<Ident>()?;
			property.push('-');
			property.push_str(&next.to_string());
		}
		input.parse::<Token![:]>()?;
		let value = input.parse()?;
		input.parse::<Token![;]>()?;
		Ok(Self { property, value })
	}
}

impl Parse for Assignment {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let variable = input.parse()?;
		input.parse::<Token![:]>()?;
		let value = input.parse()?;
		input.parse::<Token![;]>()?;
		Ok(Self { variable, value })
	}
}

impl Parse for Value {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		Ok(if input.peek_variable() {
			Self::Variable(input.parse::<Variable>()?)
		} else if input.peek(Token![.]) && input.peek2(Ident::peek_any) {
			input.parse::<Token![.]>()?;
			let name = input.parse::<Ident>()?;
			Self::ClassRef(name.to_string())
		} else if input.peek(LitStr) {
			Self::String(input.parse::<LitStr>()?.value())
		} else {
			Self::Number(input.parse::<LitInt>()?.base10_parse::<i32>()?)
		})
	}
}

impl Parse for Variable {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		input.parse::<Token![$]>()?;
		Ok(Self(input.parse::<Ident>()?))
	}
}

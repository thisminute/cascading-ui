mod initialize;
mod runtime;

use {
	data::semantics::{Semantics, StaticValue},
	proc_macro2::{Span, TokenStream},
	quote::{quote, quote_spanned},
};

fn header() -> TokenStream {
	quote! {
		extern crate wasm_bindgen;
		extern crate web_sys;
		use {
			std::{
				cell::RefCell,
				collections::HashMap,
			},
			self::wasm_bindgen::{
				prelude::*,
				JsCast,
				JsValue,
			},
			self::web_sys::{
				console,
				Event,
				HtmlElement,
				Node,
			},
		};

		#[derive(Clone, Hash, PartialEq, Eq)]
		enum Property {
			Css(&'static str),
			Link,
			Text,
			Tooltip,
			Image,
		}

		#[derive(Clone, Debug)]
		enum Value {
			Number(i32),
			String(&'static str),
			Variable(usize),
		}

		#[derive(Clone)]
		struct Effect {
			property: Property,
			target: EffectTarget,
		}

		#[derive(Clone)]
		enum EffectTarget {
			Element(HtmlElement),
			Class(&'static str),
		}

		#[derive(Clone, Default)]
		struct Group {
			class_names: Vec<&'static str>,
			selector: &'static str,

			elements: Vec<Group>,
			classes: Vec<Group>,
			listeners: Vec<Group>,
			properties: HashMap<Property, Value>,
			variables: Vec<(usize, Value)>,
		}

		trait Std {
			fn text(&self, value: Value);
			fn css(&self, property: &'static str, value: Value);
		}

		impl Std for HtmlElement {
			fn text(&self, value: Value) {
				if let Some(element) = self
					.child_nodes()
					.item(0)
				{
					element.set_node_value(None);
				}
				if let Value::String(string) = value {
					self.prepend_with_str_1(string).unwrap();
				}
			}

			fn css(&self, property: &'static str, value: Value) {
				if let Value::String(string) = value {
					self.style().set_property(property, string).unwrap();
				}
			}
		}

		thread_local! {
			static CLASSES: RefCell<HashMap<&'static str, Group>> = RefCell::new(HashMap::new());
		}
	}
}

impl Semantics {
	pub fn runtime() -> TokenStream {
		let header = header();
		let runtime_register_functions = Self::runtime_register_functions();
		let runtime_render_functions = Self::runtime_render_functions();
		quote! {
			#header
			#runtime_register_functions
			#runtime_render_functions
		}
	}

	pub fn get_mutables(&self) -> TokenStream {
		let mut mutables = vec![quote! {}; self.mutable_count];
		for (value, mutable_id) in &self.variables {
			if let &Some(mutable_id) = mutable_id {
				// if !mutables[mutable_id].is_empty() {
				// 	panic!("multiple default values for same mutable")
				// }
				let type_ = match self.get_static(value) {
					StaticValue::Number(_) => quote! { Number },
					StaticValue::String(_) => quote! { String },
					// StaticValue::Color(_, _, _, _) => quote! { String },
				};
				mutables[mutable_id] = quote! {
					Value::#type_(#value)
				};
			}
		}
		quote! { vec![ #( (#mutables, Vec::new()), )* ] }
	}

	pub fn compiled_variables(&self) -> TokenStream {
		if self.mutable_count == 0 {
			return quote! {};
		}
		let mutables = self.get_mutables();
		quote! {
			thread_local! {
				static STATE: RefCell<Vec<(Value, Vec<Effect>)>> = RefCell::new(#mutables);
			}
		}
	}

	pub fn provide_state(&self, tokens: TokenStream) -> TokenStream {
		if self.mutable_count == 0 {
			tokens
		} else {
			quote! {
				STATE.with(|state| {
					let mut state = state.borrow_mut();
					#tokens
				});
			}
		}
	}

	pub fn wasm(&self, full: bool) -> TokenStream {
		log::debug!("...Writing Wasm...");

		let warnings = self.warnings.iter().map(|error| {
			quote_spanned! {Span::call_site()=>
				compile_error!(#error);
			}
		});
		let errors = self.errors.iter().map(|error| {
			quote_spanned! {Span::call_site()=>
				compile_error!(#error);
			}
		});

		let document = self.document();

		log::debug!("Generated document");

		if document.is_empty() {
			return quote! {};
		}

		let document = self.provide_state(document);

		let core = if !self.errors.is_empty() {
			quote! {
				#( #errors )*
			}
		} else if full {
			self.full(document)
		} else {
			document
		};

		quote! {
			#( #warnings )*
			#core
		}
	}

	fn full(&self, document: TokenStream) -> TokenStream {
		let header = Self::runtime();
		let state = self.compiled_variables();
		quote! {
			#header
			#state

			extern crate console_error_panic_hook;

			#[wasm_bindgen(start)]
			pub fn run() -> Result<(), JsValue> {
				std::panic::set_hook(Box::new(console_error_panic_hook::hook));
				#document
				Ok(())
			}
		}
	}
}

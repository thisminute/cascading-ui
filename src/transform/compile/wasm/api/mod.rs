mod data_structures;
mod functions;

use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

fn imports() -> TokenStream {
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
	}
}

impl Semantics {
	pub fn runtime() -> TokenStream {
		let imports = imports();
		let data_structures = Self::runtime_data_structures();
		let functions = Self::runtime_functions();
		quote! {
			#imports
			#data_structures
			#functions
		}
	}
}

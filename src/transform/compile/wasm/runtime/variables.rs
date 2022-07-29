use {
	data::semantics::{Semantics, StaticValue},
	proc_macro2::TokenStream,
	quote::quote,
};

impl Semantics {
	pub fn runtime_state(&self) -> TokenStream {
		let variables = self
			.variables
			.iter()
			.map(|value| {
				let type_ = match value.get_static() {
					StaticValue::Number(_) => quote! { Number },
					StaticValue::String(_) => quote! { String },
					StaticValue::Color(_, _, _, _) => quote! { String },
				};
				quote! {
					Value::#type_(#value),
				}
			})
			.collect::<TokenStream>();
		quote! {
			thread_local! {
				static STATE: RefCell<Vec<Value>> = RefCell::new(vec![
					#variables
				]);
			}
		}
	}
}

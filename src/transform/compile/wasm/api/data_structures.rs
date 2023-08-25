use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

impl Semantics {
	pub fn runtime_data_structures() -> TokenStream {
		quote! {
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

			thread_local! {
				static CLASSES: RefCell<HashMap<&'static str, Group>> = RefCell::new(HashMap::new());
			}
		}
	}
}

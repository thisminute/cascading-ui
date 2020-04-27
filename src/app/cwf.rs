pub struct Runtime {
	pub window: web_sys::Window,
	pub document: web_sys::Document,
	pub body: web_sys::HtmlElement,
	pub head: web_sys::HtmlHeadElement,
}
impl Runtime {
	pub fn new() -> Self {
		Runtime {
			window: web_sys::window().expect("Failed to access global `window`."),
			document: web_sys::window().unwrap().document().expect("Failed to access `window.document`."),
			body: web_sys::window().unwrap().document().unwrap().body().expect("Failed to access `window.document.body`."),
			head: web_sys::window().unwrap().document().unwrap().head().expect("Failed to access `window.document.head`."),
		}
	}

	pub fn create_element(&self, tag: &str) -> web_sys::Element {
		self.document.create_element(&tag).unwrap()
	}

	pub fn append_child(&self, el: &web_sys::Element) -> web_sys::Node {
		self.body.append_child(el).unwrap()
	}
}

#[macro_export]
macro_rules! cwf {
	($runtime:ident
		$( // blocks
			$class:ident {
				$($rule:ident : $value:ident ;)+ // rules
			}
		)+
	) => {
		$( // for each block
			let tag = &stringify!($class);
			let element = &$runtime.create_element(tag);
			$runtime.append_child(element);
			$( // for each rule
				let text = &stringify!($value);
				element.set_inner_html(text);
			)+
		)+
	}
}

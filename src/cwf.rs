#[macro_export]
macro_rules! cwf {
	(
		$( // blocks
			$class:ident {
				$($rule:ident : $value:ident ;)+ // rules
			}
		)+
	) => {
		let window = web_sys::window().expect("Failed to access global `window`.");
		let document = window.document().expect("Failed to access `window.document`.");
		let body = document.body().expect("Failed to access `window.document.body`.");
		let _head = document.head().expect("Failed to access `window.document.head`.");

		$( // for each block
			let tag = &stringify!($class);
			let element = &document.create_element(tag).unwrap();
			body.append_child(element).unwrap();
			$( // for each rule
				let text = &stringify!($value);
				element.set_inner_html(text);
			)+
		)+
	};
}

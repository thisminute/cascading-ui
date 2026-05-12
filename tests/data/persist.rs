extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn persist_loads_from_storage() {
	let window = web_sys::window().unwrap();
	let storage = window.local_storage().unwrap().unwrap();
	storage.set_item("cui:theme", "dark").unwrap();

	test_setup! {
		let persist $theme: "light";
		text: $theme;
	}
	assert_eq!(root.inner_html(), "dark");
	storage.remove_item("cui:theme").unwrap();
}

#[wasm_bindgen_test]
fn persist_saves_on_change() {
	let window = web_sys::window().unwrap();
	let storage = window.local_storage().unwrap().unwrap();
	storage.remove_item("cui:theme").unwrap();

	test_setup! {
		let persist $theme: "light";
		text: $theme;
		?click {
			$theme: "dark";
		}
	}
	assert_eq!(root.inner_html(), "light");
	root.click();
	assert_eq!(root.inner_html(), "dark");
	assert_eq!(storage.get_item("cui:theme").unwrap().unwrap(), "dark");
	storage.remove_item("cui:theme").unwrap();
}

#[wasm_bindgen_test]
fn persist_uses_default_when_empty() {
	let window = web_sys::window().unwrap();
	let storage = window.local_storage().unwrap().unwrap();
	storage.remove_item("cui:color").unwrap();

	test_setup! {
		let persist $color: "red";
		text: $color;
	}
	assert_eq!(root.inner_html(), "red");
	storage.remove_item("cui:color").unwrap();
}

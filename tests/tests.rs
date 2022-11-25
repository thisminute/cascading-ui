extern crate wasm_bindgen_test;
use self::wasm_bindgen_test::wasm_bindgen_test_configure;
wasm_bindgen_test_configure!(run_in_browser);

mod classes {
	mod basics;
	mod event_parallel_nesting;
	mod parallel_nesting;
}
mod data {
	mod dynamic;
	mod r#static;
}
mod misc {
	mod basics;
	mod complex;
	mod events;
}
mod properties {
	mod text;
	mod title;
}

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

#[wasm_bindgen_test]
fn from_listener() {
	test_setup! {
		$text: "click me";
		text: $text;
		?click {
			$text: "hello world";
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn into_listener() {
	test_setup! {
		$text: "hello world";
		text: "click me";
		?click {
			text: $text;
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn between_listeners() {
	test_setup! {
		$text: "hello world";
		text: "click me";
		?click {
			text: $text;
		}
		a {
			?click {
				$text: "hello world";
			}
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

#[wasm_bindgen_test]
fn classes_1() {
	test_setup! {
		text: $text;
		$text: "hello world";
		?click {
			$text: "1";
		}

		a {
			text: $text;
		}
		b {
			text: $text;
			?click {
				$text: "2";
			}
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "hello world");
}

// #[wasm_bindgen_test]
// fn classes_2() {
// 	test_setup! {
// 		text: $text;
// 		$text: "hello world";
// 		button {
// 			text: "click me";
// 			?click {
// 				$text: "hello world";
// 			}
// 		}
// 		? {
// 			$text: ;
// 		}
// 		?click {
// 			.a {
// 				$text: "hello world";
// 			}
// 		}
// 		a {
// 			text: $text;
// 		}
// 		a {
// 			text: $text;
// 		}
// 	}
// }

// #[wasm_bindgen_test]
// fn classes_3() {
// 	test_setup! {
// 		text: "click me";
// 		?click {
// 			.a {
// 				$text: "hello world";
// 			}
// 		}
// 		a {
// 			text: $text;
// 		}
// 		a {
// 			text: $text;
// 		}
// 	}
// }

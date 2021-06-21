extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::*,
	web_sys::Element,
};

test_header!();

fn test(element: &Element) {
	let window = web_sys::window().unwrap();
	assert_eq!(element.inner_html(), "hello world");
	assert_eq!(
		window
			.get_computed_style(element)
			.unwrap()
			.unwrap()
			.get_property_value("color")
			.unwrap(),
		"rgb(0, 0, 255)"
	);
}

#[wasm_bindgen_test]
fn level_1() {
	test_setup! {
		.a {
			color: "blue";
		}
		a {
			text: "hello world";
		}
	}
	let element = root.first_element_child().unwrap();
	test(&element);
}

#[wasm_bindgen_test]
fn level_2() {
	test_setup! {
		.a {
			b {
				color: "blue";
			}
		}
		a {
			.b {
				text: "hello world";
			}
		}
	}
	let element = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	test(&element);
}

#[wasm_bindgen_test]
fn level_3() {
	test_setup! {
		.a {
			b {
				.c {
					color: "blue";
				}
			}
		}
		a {
			.b {
				c {
					text: "hello world";
				}
			}
		}
	}
	let element = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	test(&element);
}

#[wasm_bindgen_test]
fn level_4() {
	test_setup! {
		.a {
			b {
				.c {
					d {
						color: "blue";
					}
				}
			}
		}
		a {
			.b {
				c {
					.d {
						text: "hello world";
					}
				}
			}
		}
	}
	let element = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	test(&element);
}

#[wasm_bindgen_test]
fn level_5() {
	test_setup! {
		.a {
			b {
				.c {
					d {
						.e {
							color: "blue";
						}
					}
				}
			}
		}
		a {
			.b {
				c {
					.d {
						e {
							text: "hello world";
						}
					}
				}
			}
		}
	}
	let element = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	test(&element);
}

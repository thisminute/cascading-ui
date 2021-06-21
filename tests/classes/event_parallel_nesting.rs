extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::*,
};

test_header!();

#[wasm_bindgen_test]
fn level_1() {
	test_setup! {
		?click {
			.a {
				color: "blue";
			}
			a {
				text: "hello world";
			}
		}
	}
	assert_eq!(root.inner_html(), "");
	root.click();
	let element = root.first_element_child().unwrap();
	assert_eq!(element.inner_html(), "hello world");
	assert_eq!(
		window
			.get_computed_style(&element)
			.unwrap()
			.unwrap()
			.get_property_value("color")
			.unwrap(),
		"rgb(0, 0, 255)"
	);
}

#[wasm_bindgen_test]
fn level_2() {
	test_setup! {
		?click {
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
	}
	assert_eq!(root.inner_html(), "");
	root.click();
	let element = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	assert_eq!(element.inner_html(), "hello world");
	assert_eq!(
		window
			.get_computed_style(&element)
			.unwrap()
			.unwrap()
			.get_property_value("color")
			.unwrap(),
		"rgb(0, 0, 255)"
	);
}

#[wasm_bindgen_test]
fn level_3() {
	test_setup! {
		?click {
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
	}
	assert_eq!(root.inner_html(), "");
	root.click();
	let element = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	assert_eq!(element.inner_html(), "hello world");
	assert_eq!(
		window
			.get_computed_style(&element)
			.unwrap()
			.unwrap()
			.get_property_value("color")
			.unwrap(),
		"rgb(0, 0, 255)"
	);
}

#[wasm_bindgen_test]
fn level_4() {
	test_setup! {
		?click {
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
	}
	assert_eq!(root.inner_html(), "");
	root.click();
	let element = root
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap()
		.first_element_child()
		.unwrap();
	assert_eq!(element.inner_html(), "hello world");
	assert_eq!(
		window
			.get_computed_style(&element)
			.unwrap()
			.unwrap()
			.get_property_value("color")
			.unwrap(),
		"rgb(0, 0, 255)"
	);
}

#[wasm_bindgen_test]
fn level_5() {
	test_setup! {
		?click {
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
	}
	assert_eq!(root.inner_html(), "");
	root.click();
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
	assert_eq!(element.inner_html(), "hello world");
	assert_eq!(
		window
			.get_computed_style(&element)
			.unwrap()
			.unwrap()
			.get_property_value("color")
			.unwrap(),
		"rgb(0, 0, 255)"
	);
}

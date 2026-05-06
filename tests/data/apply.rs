// Tests for `let` declarations, variable assignment, and `apply:` property.
//
// New language features tested:
//   let $var: value  — declares a new variable in this scope
//   $var: value      — assigns to an existing variable from an ancestor scope
//   apply: .class    — listener-only property that cascades a class onto the element

extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// --- let + assignment ---

// Clicking a child element changes an ancestor's declared variable.
#[wasm_bindgen_test]
fn assignment_changes_ancestor() {
	test_setup! {
		let $color: "red";
		color: $color;
		child {
			text: "click";
			?click {
				$color: "blue";
			}
		}
	}
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
	let child = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	child.click();
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(0, 0, 255)");
}

// Assignment propagates to ALL elements referencing the variable.
#[wasm_bindgen_test]
fn assignment_propagates_to_siblings() {
	test_setup! {
		let $color: "red";
		a {
			color: $color;
			text: "a";
		}
		b {
			color: $color;
			text: "b";
		}
		c {
			text: "click";
			?click {
				$color: "blue";
			}
		}
	}
	let a = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let b = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
	let c = root.children().item(2).unwrap().dyn_into::<HtmlElement>().unwrap();
	let get_color = |el: &HtmlElement| {
		window.get_computed_style(el).unwrap().unwrap()
			.get_property_value("color").unwrap()
	};
	assert_eq!(get_color(&a), "rgb(255, 0, 0)");
	assert_eq!(get_color(&b), "rgb(255, 0, 0)");
	c.click();
	assert_eq!(get_color(&a), "rgb(0, 0, 255)");
	assert_eq!(get_color(&b), "rgb(0, 0, 255)");
}

// --- apply: ---

// `apply:` inside a listener cascades a class onto the element.
#[wasm_bindgen_test]
fn apply_cascades_class() {
	test_setup! {
		.red {
			color: "red";
			text: "i am red";
		}
		text: "click me";
		?click {
			apply: .red;
		}
	}
	assert_eq!(root.inner_html(), "click me");
	root.click();
	assert_eq!(root.inner_html(), "i am red");
	let style = window.get_computed_style(&root).unwrap().unwrap();
	assert_eq!(style.get_property_value("color").unwrap(), "rgb(255, 0, 0)");
}

// Two classes that reference each other via `apply:` cycle infinitely.
#[wasm_bindgen_test]
fn apply_replaces_listener() {
	test_setup! {
		.state_a {
			text: "A";
			?click {
				apply: .state_b;
			}
		}
		.state_b {
			text: "B";
			?click {
				apply: .state_a;
			}
		}
		state_a {}
	}
	let el = root.first_element_child().unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(el.inner_html(), "A");
	el.click();
	assert_eq!(el.inner_html(), "B");
	el.click();
	assert_eq!(el.inner_html(), "A");
	el.click();
	assert_eq!(el.inner_html(), "B");
}

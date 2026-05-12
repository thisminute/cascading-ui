extern crate cascading_ui;
extern crate wasm_bindgen_test;
use self::{
	cascading_ui::{test_header, test_setup},
	wasm_bindgen_test::wasm_bindgen_test,
};

test_header!();

// Route blocks create container elements with data-route attributes
#[wasm_bindgen_test]
fn route_creates_elements() {
	test_setup! {
		/home {
			text: "Home";
		}
		/about {
			text: "About";
		}
	}
	// Both routes should be in the DOM
	let home = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let about = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
	assert_eq!(home.inner_html(), "Home");
	assert_eq!(about.inner_html(), "About");
	assert_eq!(home.get_attribute("data-route").unwrap(), "home");
	assert_eq!(about.get_attribute("data-route").unwrap(), "about");
}

// First route should be visible by default, others hidden
#[wasm_bindgen_test]
fn route_default_visibility() {
	// Clean hash state
	let w = web_sys::window().unwrap();
	w.location().set_hash("").unwrap();

	test_setup! {
		/home {
			text: "Home";
		}
		/about {
			text: "About";
		}
	}
	let home = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let about = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();

	let home_display = window.get_computed_style(&home).unwrap().unwrap()
		.get_property_value("display").unwrap();
	let about_display = window.get_computed_style(&about).unwrap().unwrap()
		.get_property_value("display").unwrap();

	assert_ne!(home_display, "none");
	assert_eq!(about_display, "none");
}

// Setting hash before init selects the correct route
#[wasm_bindgen_test]
fn route_initial_hash() {
	// Set hash BEFORE test_setup so the router picks it up on init
	let w = web_sys::window().unwrap();
	w.location().set_hash("about").unwrap();

	test_setup! {
		/home {
			text: "Home";
		}
		/about {
			text: "About";
		}
	}
	let home = root.children().item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
	let about = root.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();

	// About should be visible, home hidden
	let home_display = home.style().get_property_value("display").unwrap();
	let about_display = about.style().get_property_value("display").unwrap();
	assert_eq!(home_display, "none");
	assert_ne!(about_display, "none");

	// Clean up
	w.location().set_hash("").unwrap();
}

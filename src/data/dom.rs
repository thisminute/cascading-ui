// struct Attribute<'a> {
// 	name: &'a str,
// 	values: Vec<&'a str>,
// }

pub struct Element<'a> {
	// pub classes: Vec<Class>,
	pub active: bool,
	pub children: Vec<Element<'a>>,
	pub text: &'a str,
	pub link: bool,
}
// impl Default for Element {
// 	fn default() -> Self {
// 		Self {
// 			// tag: "div",
// 			// attributes: Vec::new(),
// 			children: Vec::new(),
// 			// classes: Vec::new(),
// 		}
// 	}
// }
impl Element<'_> {
	pub fn new() -> Self {
		Self {
			active: false,
			children: Vec::new(),
			text: "",
			link: false,
		}
	}
}

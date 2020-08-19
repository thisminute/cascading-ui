// struct Attribute<'a> {
// 	name: &'a str,
// 	values: Vec<&'a str>,
// }

pub struct Element {
	// pub classes: Vec<Class>,
	pub active: bool,
	pub children: Vec<Element>,
	pub text: String,
	pub link: Option<String>,
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
impl Element {
	pub fn new() -> Self {
		Self {
			active: false,
			children: Vec::new(),
			text: "".to_string(),
			link: None,
		}
	}
}

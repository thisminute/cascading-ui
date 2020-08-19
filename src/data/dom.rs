// struct Attribute<'a> {
// 	name: &'a str,
// 	values: Vec<&'a str>,
// }
pub enum Event {
	Click,
}

pub struct Element {
	// pub classes: Vec<Class>,
	pub active: bool,
	pub children: Vec<Element>,
	pub link: Option<String>,
	pub listeners: Vec<Event>,
	pub text: String,
	pub tooltip: Option<String>,
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
			listeners: Vec::new(),
			tooltip: None,
		}
	}
}

// struct Attribute<'a> {
// 	name: &'a str,
// 	values: Vec<&'a str>,
// }

pub struct Element {
	// pub classes: Vec<Class>,
	pub children: Vec<Element>,
}
impl Default for Element {
	fn default() -> Self {
		Self {
			// tag: "div",
			// attributes: Vec::new(),
			children: Vec::new(),
			// classes: Vec::new(),
		}
	}
}

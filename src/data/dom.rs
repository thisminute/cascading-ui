// struct Attribute<'a> {
// 	name: &'a str,
// 	values: Vec<&'a str>,
// }

pub struct Element<'a> {
	// pub classes: Vec<Class>,
	pub text: &'a str,
	pub children: Vec<Element<'a>>,
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
// impl Element {
// 	// pub fn get_element_at_path(&self, path: Vec<usize>) -> &Element {
// 	// 	let mut current = self;
// 	// 	for index in path {
// 	// 		current = &current.children[index];
// 	// 	}
// 	// 	current
// 	// }
// }

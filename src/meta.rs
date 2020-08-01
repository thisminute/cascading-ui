use std::collections::HashMap;

pub struct List<'a> {
	id: &'a str,
	next: &'a Option<List<'a>>,
}

pub struct Context {
	// pub path: Option<List<'a>>,
// pub r#type: Prefix,
}

struct Class<'a> {
	text: &'a str,
	styles: Vec<&'a str>,
}
impl Default for Class<'_> {
	fn default() -> Self {
		Class {
			text: "",
			styles: Vec::new(),
		}
	}
}
pub struct Meta<'a> {
	title: Option<&'a str>,
	classes: HashMap<&'a str, Class<'a>>,
}

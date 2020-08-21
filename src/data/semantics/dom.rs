// use std::sync::atomic::AtomicUsize;

pub enum Event {
	Click,
}

// static COUNTER = AtomicUsize::new(1);
pub struct Element {
	// pub classes: Vec<Class>,
	pub active: bool,
	pub children: Vec<Element>,
	pub listeners: Vec<Event>,

	pub id: Option<String>,
	pub link: Option<String>,
	pub text: String,
	pub tooltip: Option<String>,
}

impl Element {
	pub fn new() -> Self {
		Self {
			active: false,
			children: Vec::new(),
			listeners: Vec::new(),

			id: None,
			link: None,
			text: "".to_string(),
			tooltip: None,
		}
	}
}

pub enum Event {
	Click,
}

pub struct Effects {
	pub text: Option<String>,
	pub link: Option<String>,
}
// impl Effects {
// 	fn new() -> Self {
// 		Self {
// 			text: None,
// 			link: None,
// 		}
// 	}
// }

// pub struct Action {}

pub struct EventListener {
	pub event: Event,
	pub selector: String,
	pub effects: Effects,
}

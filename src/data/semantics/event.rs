// pub enum Event {
// 	Click,
// }

pub struct Effects {
	pub text: Option<String>,
	pub link: Option<String>,
}

pub struct EventListener {
	// pub event: Event,
	pub selector: String,
	pub effects: Effects,
}

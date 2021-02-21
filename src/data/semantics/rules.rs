#[derive(Default, Clone)]
pub struct Rules {
	pub background_color: Option<String>,
	pub color: Option<String>,
	pub link: Option<String>,
	pub text: Option<String>,
	pub route: Option<String>,
	pub tooltip: Option<String>,
}
impl Rules {
	pub fn new() -> Self {
		Self {
			background_color: None,
			color: None,
			link: None,
			text: None,
			route: None,
			tooltip: None,
		}
	}
}

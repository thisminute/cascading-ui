use data::{
	dom::{Dom, Element, Page},
	Semantics,
};

impl Dom {
	pub fn html(&self, semantics: &mut Semantics) -> Vec<(String, String)> {
		vec![if self.html_roots.contains_key("/") {
			(
				String::from("index.html"),
				self.html_roots["/"].html(semantics),
			)
		} else {
			panic!("no / route found");
		}]
	}
}

impl Page {
	fn html(&self, semantics: &Semantics) -> String {
		let styles = [
			("background-color", &self.root.rules.background_color),
			("color", &self.root.rules.color),
		]
		.iter()
		.filter(|(_, value)| value.is_some())
		.map(|(property, value)| format!("{}:{}", property, value.as_ref().unwrap()))
		.collect::<Vec<String>>()
		.join(";");

		let attributes = [("style", styles)]
			.iter()
			.filter(|(_, value)| value.len() > 0)
			.map(|(property, value)| format!(" {}='{}'", property, value))
			.collect::<Vec<String>>()
			.join("");

		let content = {
			let text = self.root.rules.text.clone().unwrap_or_default();
			let children = self
				.root
				.children
				.iter()
				.map(|child| child.html(semantics))
				.collect::<Vec<String>>()
				.join("");
			if let Some(href) = &self.root.rules.link {
				format!("<a href='{}'>{}{}</a>", href, text, children)
			} else {
				format!("{}{}", text, children)
			}
		};

		format!(
			"<html>{}{}</html>",
			format!("<head>{}{}</head>", 
				format!("<title>{}</title>", self.title),
				format!("<style>{}</style>", self.title)
			),
			format!(
				"<body{}>{}{}{}</body>",
				attributes,
				"<noscript>This page contains Webassembly and Javascript content. Please make sure that you are using the latest version of a modern browser and that Javascript and Webassembly (Wasm) are enabled.</noscript>",
				content,
				"<script src='./bootstrap.js'></script>"
			)
		)
	}
}

impl Element {
	fn html(&self, semantics: &Semantics) -> String {
		let tag = if self.rules.link.is_some() {
			"a"
		} else {
			"div"
		};
		let styles = [
			("background-color", &self.rules.background_color),
			("color", &self.rules.color),
		]
		.iter()
		.filter(|(_, value)| value.is_some())
		.map(|(property, value)| format!("{}:{}", property, value.as_ref().unwrap()))
		.collect::<Vec<String>>()
		.join(";");

		let attributes = [
			("style", styles),
			("href", self.rules.link.clone().unwrap_or_default()),
		]
		.iter()
		.filter(|(_, value)| value.len() > 0)
		.map(|(property, value)| format!(" {}='{}'", property, value))
		.collect::<Vec<String>>()
		.join("");

		let children = self
			.children
			.iter()
			.map(|child| child.html(semantics))
			.collect::<Vec<String>>()
			.join("");

		format!(
			"<{0}{1}>{2}{3}</{0}>",
			tag,
			attributes,
			self.rules.text.clone().unwrap_or(format!("")),
			children
		)
	}
}

use {
	data::dom::{Dom, Element, Page},
	transform::write::css::Css,
};

impl Dom {
	pub fn html(&self) -> Vec<(String, String)> {
		vec![if self.html_roots.contains_key("/") {
			(String::from("index.html"), self.html_roots["/"].html())
		} else {
			panic!("no / route found");
		}]
	}
}

impl Page {
	fn html(&self) -> String {
		format!(
			"<html>{}{}</html>",
			format!("<head>{}{}</head>", 
				format!("<title>{}</title>", self.title),
				format!("<style>{}</style>", self.title)
			),
			format!(
				"<body>{}{}{}</body>",
				"<noscript>This page contains Webassembly and Javascript content. Please make sure that you are using the latest version of a modern browser and that Javascript and Webassembly (Wasm) are enabled.</noscript>",
				self.root.html(),
				"<script src='./bootstrap.js'></script>"
			)
		)
	}
}

impl Element {
	fn html(&self) -> String {
		let tag = if self.link.is_some() { "a" } else { "div" };
		let attributes = [
			("style", self.style.css()),
			("class", self.classes.join(" ")),
			("href", self.link.clone().unwrap_or_default()),
		]
		.iter()
		.filter(|(_, value)| value.len() > 0)
		.map(|(attribute, value)| format!(" {}='{}'", attribute, value))
		.collect::<Vec<String>>()
		.join("");

		let children = self
			.children
			.iter()
			.map(|child| child.html())
			.collect::<Vec<String>>()
			.join("");

		format!("<{0}{1}>{2}{3}</{0}>", tag, attributes, self.text, children)
	}
}

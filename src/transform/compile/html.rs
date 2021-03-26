use {
	data::semantics::{properties::CwlProperty, Group, Page, Semantics},
	std::collections::HashMap,
	transform::compile::css::Css,
};

impl Semantics {
	pub fn html(&self) -> HashMap<String, String> {
		self.pages
			.iter()
			.map(|page| (page.route.clone(), page.html(&self.groups)))
			.collect()
	}
}

impl Page {
	fn html(&self, groups: &Vec<Group>) -> String {
		format!(
			"<html>{}{}</html>",
			format!("<head>{}{}</head>", 
				format!("<title>{}</title>", self.title),
				format!("<style>{}</style>", self.styles.css())
			),
			format!(
				"<body>{}{}{}</body>",
				"<noscript>This page contains Webassembly and Javascript content. Please make sure that you are using the latest version of a modern browser and that Javascript and Webassembly (Wasm) are enabled.</noscript>",
				groups[self.root_id].html(groups),
				"<script src='./bootstrap.js'></script>"
			)
		)
	}
}

impl Group {
	fn html(&self, groups: &Vec<Group>) -> String {
		let link = match self.properties.cwl.get(&CwlProperty::Link) {
			Some(value) => value,
			None => "",
		};
		let attributes = [
			("style", &*self.properties.css.css()),
			("class", &*self.class_names.join(" ")),
			("href", link),
		]
		.iter()
		.filter(|(_, value)| !value.is_empty())
		.map(|(attribute, value)| format!(" {}='{}'", attribute, value))
		.collect::<Vec<String>>()
		.join("");

		let children = self
			.elements
			.iter()
			.map(|&child_id| groups[child_id].html(groups))
			.collect::<Vec<String>>()
			.join("");

		format!(
			"<{0}{1}>{2}{3}</{0}>",
			self.tag(),
			attributes,
			match self.properties.cwl.get(&CwlProperty::Text) {
				Some(value) => value,
				None => "",
			},
			children
		)
	}
}

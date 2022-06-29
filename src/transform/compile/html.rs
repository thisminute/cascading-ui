use {
	data::semantics::{properties::CuiProperty, Group, Semantics},
	std::collections::HashMap,
	transform::compile::css::Css,
};

impl Semantics {
	pub fn html(&self) -> (String, HashMap<String, String>) {
		log::debug!("...Writing HTML...");
		let (contents, styles) = self.html_parts();
		let homepage = contents.get("/").unwrap();
		let root = &self.pages[self.pages[0].root_id];
		// TODO: make this cleaner with a lightweight html!() macro
		let html = format!(
			"<html>{}{}</html>",
			format_args!("<head>{}{}</head>",
				format_args!("<title>{}</title>", root.title),
				format_args!("<style>{}</style>", styles)
			),
			format_args!(
				"<body>{}{}{}</body>",
				homepage,
				"<noscript>This page contains Webassembly and Javascript content. Please make sure that you are using the latest version of a modern browser and that Javascript and Webassembly (Wasm) are enabled.</noscript>",
				format_args!(
					"<script type=\"module\">{}{}</script>",
					"import init from './cui/cui_app_template.js';",
					"init();"
				)
			)
		);
		(html, contents)
	}

	pub fn html_parts(&self) -> (HashMap<String, String>, String) {
		let contents = self
			.pages
			.iter()
			.map(|page| {
				(
					page.route.clone(),
					self.groups[page.root_id].html(&self.groups),
				)
			})
			.collect::<HashMap<String, String>>();
		(contents, self.styles.css())
	}
}

impl Group {
	fn html(&self, groups: &[Group]) -> String {
		let link = if let Some(value) = self.properties.cui.get(&CuiProperty("link".to_string())) {
			value.get_string()
		} else {
			"".into()
		};
		let attributes = [
			("style", &*self.properties.css.css()),
			("class", &*self.class_names.join(" ")),
			("href", &*link),
		]
		.iter()
		.filter(|(_, value)| !value.is_empty())
		.map(|(attribute, value)| format!(" {}='{}'", attribute, value))
		.collect::<Vec<String>>()
		.join("");

		let children = self
			.elements
			.iter()
			.filter(|&&element_id| groups[element_id].is_static())
			.map(|&child_id| groups[child_id].html(groups))
			.collect::<Vec<String>>()
			.join("");

		let contents = format!(
			"{}{}",
			if let Some(value) = self.properties.cui.get(&CuiProperty("text".to_string())) {
				value.get_string()
			} else {
				"".into()
			},
			children
		);

		format!("<{0}{1}>{2}</{0}>", self.tag, attributes, contents)
	}
}

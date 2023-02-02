use {
	data::semantics::{
		properties::{CuiProperty, Property},
		Group, Semantics, StaticValue, Value,
	},
	std::collections::HashMap,
	transform::compile::css::Css,
};

impl Semantics {
	pub fn html(&self, static_: bool) -> (String, HashMap<&str, String>) {
		log::debug!("...Writing HTML...");
		let (contents, styles) = self.html_parts();
		let homepage = contents.get("/").unwrap();
		let root = &self.pages[self.pages[0].root_id];
		// TODO: make this cleaner with a lightweight html!() macro

		let html = format!(
			"<html>{}{}</html>",
			format_args!(
				"<head>{}{}</head>",
				format_args!("<title>{}</title>", root.title),
				format_args!("<style>{}</style>", styles)
			),
			format_args!(
				"<body>{}{}</body>",
				homepage,
				if static_ {
					"".into()
				} else {
					format!("{}{}",
						"<noscript>This page contains Webassembly and Javascript content. Please make sure that you are using the latest version of a modern browser and that Javascript and Webassembly (Wasm) are enabled.</noscript>",
						format_args!(
							"<script type=\"module\">{}{}</script>",
							"import init from './cui/cui_app_template.js';",
							"init();"
						)
					)
				}
			),
		);
		(html, contents)
	}

	pub fn html_parts(&self) -> (HashMap<&str, String>, String) {
		let contents = (self.pages.iter())
			.map(|page| (page.route, self.groups[page.root_id].html(&self.groups)))
			.collect();
		(contents, self.styles.css())
	}
}

impl Group {
	fn html(&self, groups: &[Group]) -> String {
		let link = self.properties.get(&Property::Cui(CuiProperty::Link));
		let attributes = [
			("style", &*self.properties.css()),
			("class", &*self.class_names.join(" ")),
			(
				"href",
				&*link
					.unwrap_or(&Value::Static(StaticValue::String("".to_string())))
					.to_string(),
			),
		]
		.iter()
		.filter(|(_, value)| !value.is_empty())
		.map(|(attribute, value)| format!(" {}='{}'", attribute, value))
		.collect::<String>();

		let children = (self.elements.iter())
			.filter(|&&element_id| groups[element_id].is_compiled())
			.map(|&child_id| groups[child_id].html(groups))
			.collect::<String>();

		let contents = format!(
			"{}{}",
			if let Some(value) = self.properties.get(&Property::Cui(CuiProperty::Text)) {
				value.to_string()
			} else {
				"".into()
			},
			children
		);

		format!("<{0}{1}>{2}</{0}>", self.tag, attributes, contents)
	}
}

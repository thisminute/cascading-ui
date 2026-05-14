use {
	crate::data::semantics::{
		properties::{CuiProperty, Property},
		Group, Semantics, StaticValue, Value,
	},
	std::collections::HashMap,
	crate::transform::compile::css::Css,
};

impl Semantics {
	pub fn html(&self, static_: bool) -> (String, HashMap<&str, String>) {
		log::debug!("...Writing HTML...");
		let (contents, styles) = self.html_parts();
		let homepage = contents.get("/").unwrap();
		let root = &self.pages[self.pages[0].root_id];
		// TODO: make this cleaner with a lightweight html!() macro

		let html = format!(
			"<!DOCTYPE html><html lang='en'>{}{}</html>",
			format_args!(
				"<head>{}{}{}{}</head>",
				"<meta charset='UTF-8'>",
				"<meta name='viewport' content='width=device-width, initial-scale=1.0'>",
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
						{
						let pkg_name = std::env::var("CARGO_PKG_NAME")
							.unwrap_or_else(|_| "cui_app_template".to_string())
							.replace('-', "_");
						format!(
							"<script type=\"module\">{}{}</script>",
							format!("import init from './cui/{}.js';", pkg_name),
							"init();"
						)
					}
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
		.map(|(attribute, value)| {
			let escaped = value
				.replace('&', "&amp;")
				.replace('\'', "&#39;")
				.replace('"', "&quot;")
				.replace('<', "&lt;")
				.replace('>', "&gt;");
			format!(" {}='{}'", attribute, escaped)
		})
		.collect::<String>();

		let children = (self.elements.iter())
			.filter(|&&element_id| groups[element_id].is_compiled())
			.map(|&child_id| groups[child_id].html(groups))
			.collect::<String>();

		let contents = format!(
			"{}{}",
			if let Some(value) = self.properties.get(&Property::Cui(CuiProperty::Text)) {
				value.to_string()
					.replace('&', "&amp;")
					.replace('<', "&lt;")
					.replace('>', "&gt;")
			} else {
				"".into()
			},
			children
		);

		format!("<{0}{1}>{2}</{0}>", self.tag, attributes, contents)
	}
}

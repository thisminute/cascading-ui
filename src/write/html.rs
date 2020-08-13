use {
	crate::BoxResult,
	data::{dom::Element, Semantics},
	html_minifier::HTMLMinifier,
};

pub trait Html {
	fn html(&self, minifier: &mut HTMLMinifier) -> BoxResult<()>;
}

fn recurse(element: &Element, minifier: &mut HTMLMinifier) -> BoxResult<()> {
	minifier.digest("<div>")?;
	minifier.digest(element.text)?;
	minifier.digest("</div>")?;
	for child in &element.children {
		recurse(&child, minifier)?;
	}
	Ok(())
}

impl Html for Semantics<'_> {
	fn html(&self, minifier: &mut HTMLMinifier) -> BoxResult<()> {
		minifier.digest(
			"
		<!DOCTYPE html>
		<html>
			<head>
				<meta charset='utf-8'>
		",
		)?;
		match &self.title {
			Some(title) => {
				let title = title.to_string();
				let length = title.len() - 1;
				minifier.digest("<title>")?;
				minifier.digest(&title[1..length])?;
				minifier.digest("</title>")?;
			}
			None => {}
		}
		minifier.digest("
			</head>
			<body>
				<noscript>This page contains webassembly and javascript content, please enable javascript in your browser and make sure you are using the latest version of a popular modern browser.</noscript>
		")?;
		match &self.dom {
			Some(dom) => recurse(dom, minifier)?,
			None => {}
		}
		minifier.digest(
			"
				<script src='./bootstrap.js'></script>
			</body>
		</html>
		",
		)?;
		Ok(())
	}
}

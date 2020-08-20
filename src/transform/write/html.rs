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
	minifier.digest(element.text.clone())?;
	for child in &element.children {
		if child.active {
			recurse(child, minifier)?;
		}
	}
	minifier.digest("</div>")?;
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
				minifier.digest("<title>")?;
				minifier.digest(title.clone())?;
				minifier.digest("</title>")?;
			}
			None => {}
		}
		minifier.digest("
			</head>
			<body>
				<noscript>This page contains webassembly and javascript content, please enable javascript in your browser and make sure you are using the latest version of a popular modern browser.</noscript>
		")?;
		recurse(&self.dom, minifier)?;
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

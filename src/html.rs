use {crate::BoxResult, html_minifier::HTMLMinifier, meta::Meta, tokens::*};

pub trait Html {
	fn html(&self, meta: &Meta, minifier: &mut HTMLMinifier) -> BoxResult<()>;
}

impl Html for Document {
	fn html(&self, meta: &Meta, minifier: &mut HTMLMinifier) -> BoxResult<()> {
		let mut h = |s: &str| minifier.digest(s).unwrap();
		h("
		<!DOCTYPE html>
		<html>
			<head>
				<meta charset='utf-8'>
		");

		match &meta.title {
			Some(title) => {
				let title = title.to_string();
				let length = title.len() - 1;
				h("<title>");
				h(&title[1..length]);
				h("</title>");
			}
			None => {}
		}

		// yew_macro::html!(
		// 	<html>
		// 		<head>

		// 		<head>
		// 	</body>
		// );

		h("
			</head>
			<body>
				<noscript>This page contains webassembly and javascript content, please enable javascript in your browser and make sure you are using the latest version of a popular modern browser.</noscript>
				<script src='./bootstrap.js'></script>
			</body>
		</html>
		");
		Ok(())
	}
}

impl Html for Block {
	fn html(&self, _meta: &Meta, minifier: &mut HTMLMinifier) -> BoxResult<()> {
		let mut _h = |s: &str| minifier.digest(s).unwrap();

		match self.prefix {
			Prefix::Instance => {}
			Prefix::Class => {}
			Prefix::Action => {}
			Prefix::Listener => {}
		}

		Ok(())
	}
}

// impl Html for Rule {
//		fn html(&self, meta: &Meta, minifier: &mut HTMLMinifier) -> BoxResult<()> {
// 		minifier.digest("<body>hello world</body>");
// 		minifier.get_html()
// 	}
// }

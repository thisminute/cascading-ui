use {crate::tokens::*, html_minifier::HTMLMinifier};

pub trait Html {
	fn html(&self, minifier: &mut HTMLMinifier);
}

impl Html for Document {
	fn html(&self, minifier: &mut HTMLMinifier) {
		minifier.digest("<body>hello world</body>").unwrap();
	}
}

// impl Html for Block {
// 	fn html(&self, minifier: &mut HTMLMinifier) {
// 		minifier.digest("<body>hello world</body>");
// 		minifier.get_html()
// 	}
// }

// impl Html for Rule {
// 	fn html(&self, minifier: &mut HTMLMinifier) {
// 		minifier.digest("<body>hello world</body>");
// 		minifier.get_html()
// 	}
// }

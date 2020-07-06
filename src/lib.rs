use cwf::cwf;

cwf! {
	title: "Stack Overflow";

	header {
		text: "S/O";
		hamburger {}
		logo {}
		products {
			text: "Products";
			searchbar {}
		}
		search {}
		icons {
			profile {}
			inbox {}
			achievements {}
			review {}
			help {}
			// site-switcher {}
		}
	}
	content {
		mainbar {
			headline {}
			filter {}
			list {}
		}
		sidebar {
			stuff {}
		}
	}
}

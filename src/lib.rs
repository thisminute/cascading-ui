use cwf::cwf;

cwf! {
	title: "Stack Overflow";

	header {
		text: "S/O";
		background_color: "yellow";
		color: "blue";
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

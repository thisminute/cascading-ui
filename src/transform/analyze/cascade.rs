use data::semantics::Group;

type Groups = Vec<Group>;

pub trait Cascade {
	fn cascade(&mut self, from_group_id: usize, into_group_id: usize);
	fn cascade_css(&mut self, from_group_id: usize, into_group_id: usize);
}

impl Cascade for Groups {
	fn cascade(&mut self, from_group_id: usize, into_group_id: usize) {
		eprintln!(
			"Cascading from group {} into group {}",
			from_group_id, into_group_id
		);
		if from_group_id == into_group_id {
			return;
		}

		for (&property, value) in &self[from_group_id].properties.cwl.clone() {
			eprintln!(" Cascading cwl property {:?}:{}", property, value);
			self[into_group_id]
				.properties
				.cwl
				.entry(property)
				.or_insert(value.clone());
		}
		for _ in &self[from_group_id].properties.page {
			panic!("page properties should never be cascaded");
		}
		for (name, class_ids) in &self[from_group_id].classes.clone() {
			for &class_id in class_ids {
				eprintln!(" Cascading scoped group with name {}", name);
				let classes = self[into_group_id].classes.entry(name.clone()).or_default();
				classes.push(class_id);
			}
		}
		if self[from_group_id].elements.len() > 0 {
			eprintln!(
				" Cascading element rules {:?} into {:?}",
				self[from_group_id].elements, self[into_group_id].elements
			);
			if self[into_group_id].elements.len() > 0 {
				panic!("can't disambiguate which elements get appended")
			}
			self[into_group_id].elements = self[from_group_id].elements.clone();
		}
	}

	fn cascade_css(&mut self, from_group_id: usize, into_group_id: usize) {
		for (&property, value) in &self[from_group_id].properties.css.clone() {
			eprintln!(" Cascading css property {:?}:{}", property, value);
			self[into_group_id]
				.properties
				.css
				.entry(property)
				.or_insert(value.clone());
		}
	}
}

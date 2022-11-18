use crate::structure::{GValue, List};

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
	labels: Box<GValue>,
	objects: List,
}

impl Path {
	pub fn new(labels: GValue, objects: List) -> Self {
		Path {
			labels: Box::new(labels),
			objects,
		}
	}

	pub fn objects(&self) -> &List {
		&self.objects
	}
}

use std::collections::HashMap;

use uuid::Uuid;

use crate::Error;

/// # Property Vertex
/// Vertices are also referred to as node or points.
#[derive(Default, Debug, PartialEq)]
pub struct Vertex {
	pub id: Uuid,
	pub labels: Vec<Uuid>,
	pub props: HashMap<Uuid, Vec<u8>>,
}

impl Vertex {
	pub fn new(labels: Vec<Uuid>, props: HashMap<Uuid, Vec<u8>>) -> Result<Self, Error> {
		let mut vertex = Vertex {
			id: Uuid::new_v4(),
			labels: Vec::default(),
			props,
		};

		vertex.add_labels(labels).unwrap();

		Ok(vertex)
	}

	pub fn add_label(&mut self, label: Uuid) -> Result<(), Error> {
		self.labels.push(label);
		Ok(())
	}

	pub fn add_labels(&mut self, labels: Vec<Uuid>) -> Result<(), Error> {
		labels.iter().for_each(|l| self.add_label(*l).unwrap());
		Ok(())
	}
}

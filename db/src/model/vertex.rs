use std::collections::{HashSet, LinkedList};

use uuid::Uuid;

use crate::Error;

/// # Property Vertex
/// Vertices are also referred to as node or points.
#[derive(Default)]
pub struct Vertex {
	pub labels: LinkedList<Uuid>,
	pub props: HashSet<Uuid, Vec<u8>>,
}

impl Vertex {
	pub fn new(labels: Vec<Uuid>, props: HashSet<Uuid, Vec<u8>>) -> Result<Self, Error> {
		let mut vertex = Vertex {
			labels: LinkedList::default(),
			props,
		};

		vertex.add_labels(labels).unwrap();

		Ok(vertex)
	}

	pub fn add_label(&mut self, label: Uuid) -> Result<(), Error> {
		self.labels.push_back(label);
		Ok(())
	}

	pub fn add_labels(&mut self, labels: Vec<Uuid>) -> Result<(), Error> {
		labels.iter().for_each(|l| self.add_label(*l).unwrap());
		Ok(())
	}
}

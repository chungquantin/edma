use std::collections::{HashSet, LinkedList};

use uuid::Uuid;

use crate::Error;

/// # Property Node
/// Nodes are also referred to as vertices or points.
#[derive(Default)]
pub struct Node {
	pub labels: LinkedList<Uuid>,
	pub props: HashSet<Uuid, Vec<u8>>,
}

impl Node {
	pub fn new(labels: Vec<Uuid>, props: HashSet<Uuid, Vec<u8>>) -> Result<Self, Error> {
		let mut node = Node {
			labels: LinkedList::default(),
			props,
		};

		node.add_labels(labels).unwrap();

		Ok(node)
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

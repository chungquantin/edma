use std::collections::HashMap;

use uuid::Uuid;

use crate::{ControllerError, Error, Label};

pub const MAX_LABELS: u8 = 5;

/// # Property Vertex
/// Vertices are also referred to as node or points.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Vertex {
	pub id: Uuid,
	pub labels: Vec<Uuid>,
	pub props: HashMap<Uuid, Vec<u8>>,
}

impl Vertex {
	pub fn new(labels: Vec<Label>, props: HashMap<Uuid, Vec<u8>>) -> Result<Self, Error> {
		let mut vertex = Vertex {
			id: Uuid::new_v4(),
			labels: Vec::default(),
			props,
		};

		vertex.add_labels(labels).unwrap();

		Ok(vertex)
	}

	pub fn add_label(&mut self, label: &Label) -> Result<(), ControllerError> {
		self.validate_max_labels(1);
		self.labels.push(label.id);
		Ok(())
	}

	pub fn add_labels(&mut self, labels: Vec<Label>) -> Result<(), ControllerError> {
		self.validate_max_labels(labels.len());
		labels.iter().for_each(|l| self.add_label(l).unwrap());
		Ok(())
	}

	fn validate_max_labels(&self, add: usize) {
		if self.labels.len() + add > MAX_LABELS.into() {
			panic!("{}", ControllerError::ExceedMaxLabel);
		}
	}
}

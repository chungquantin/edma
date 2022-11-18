use serde_json::Value;
use uuid::Uuid;

use crate::{err::ControllerError, util::get_now, Error, Identifier, Label};

const MAX_LABELS: u8 = 1;

/// ## Relationship
/// Relationships are also referred to as edges, links, or lines.
/// - Relationships describes a connection between a source node and a target node.
/// - Relationships always has a direction (one direction).
/// - Relationships must have a type (one type) to define (classify) what type of relationship they are.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Edge {
	/// Source vertex (inbound)
	pub in_id: Uuid,
	/// Relationship type
	pub t: Identifier,
	/// Target vertex (outbound)
	pub out_id: Uuid,
	/// Timestamp
	pub timestamp: i64,
	//// Labels
	pub labels: Vec<Uuid>,
	/// Properties
	pub props: Value,
}

impl Edge {
	pub fn new(in_id: Uuid, t: Identifier, out_id: Uuid) -> Result<Self, Error> {
		Ok(Edge {
			in_id,
			out_id,
			t,
			timestamp: get_now(),
			labels: Vec::default(),
			props: Value::default(),
		})
	}

	pub fn add_props(&mut self, props: Value) -> Result<(), ControllerError> {
		self.props = props;
		Ok(())
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

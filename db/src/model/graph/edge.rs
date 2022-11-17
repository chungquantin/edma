use serde_json::Value;
use uuid::Uuid;

use crate::{util::get_now, Error, Identifier};

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
	/// Properties
	pub props: Value,
}

impl Edge {
	pub fn new(in_id: Uuid, t: Identifier, out_id: Uuid, props: Value) -> Result<Self, Error> {
		Ok(Edge {
			in_id,
			out_id,
			t,
			timestamp: get_now(),
			props,
		})
	}
}

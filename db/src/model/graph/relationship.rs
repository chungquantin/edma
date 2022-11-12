use std::collections::HashMap;

use uuid::Uuid;

use crate::{util::get_now, Error, Identifier};

/// ## Relationship
/// Relationships are also referred to as edges, links, or lines.
/// - Relationships describes a connection between a source node and a target node.
/// - Relationships always has a direction (one direction).
/// - Relationships must have a type (one type) to define (classify) what type of relationship they are.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Relationship {
	/// Source vertex (inbound)
	pub source: Uuid,
	/// Target vertex (outbound)
	pub target: Uuid,
	/// Relationship type
	pub t: Identifier,
	/// Timestamp
	pub timestamp: i64,
	/// Properties
	pub props: HashMap<Uuid, Vec<u8>>,
}

impl Relationship {
	pub fn new(
		source: Uuid,
		target: Uuid,
		t: Identifier,
		props: HashMap<Uuid, Vec<u8>>,
	) -> Result<Self, Error> {
		Ok(Relationship {
			source,
			target,
			t,
			timestamp: get_now(),
			props,
		})
	}
}

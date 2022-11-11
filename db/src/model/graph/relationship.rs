use std::collections::HashMap;

use uuid::Uuid;

use crate::{util::get_now, Error, Identifier};

/// ## Relationship
/// Relationships are also referred to as edges, links, or lines.
/// - Relationships describes a connection between a source node and a target node.
/// - Relationships always has a direction (one direction).
/// - Relationships must have a type (one type) to define (classify) what type of relationship they are.
#[derive(Debug, Clone, Default)]
pub struct Relationship {
	pub id: Uuid,
	/// Source node (outgoing)
	pub s_node: Uuid,
	/// Target node (incoming)
	pub t_node: Uuid,
	/// Relationship type
	pub t: Identifier,
	/// Timestamp
	pub timestamp: i64,
	/// Properties
	pub props: HashMap<Uuid, Vec<u8>>,
}

impl Relationship {
	pub fn new(
		s_node: Uuid,
		t_node: Uuid,
		t: Identifier,
		props: HashMap<Uuid, Vec<u8>>,
	) -> Result<Self, Error> {
		Ok(Relationship {
			id: Uuid::new_v4(),
			s_node,
			t_node,
			t,
			timestamp: get_now(),
			props,
		})
	}
}

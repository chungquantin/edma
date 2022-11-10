use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Error;

#[derive(PartialEq, Serialize, Deserialize, Eq, Debug, Clone)]
pub enum PropertyVariant {
	Unknown = 0,
	String = 1,
	UInt32 = 2,
	UInt64 = 3,
	UInt128 = 4,
	Document = 5,
	VecString = 6,
	VecUint32 = 7,
	VecUint64 = 8,
	VecUint128 = 9,
}

impl Default for PropertyVariant {
	fn default() -> Self {
		PropertyVariant::Unknown
	}
}

/// ## Property
/// Nodes and relationships can have properties (key-value pairs),
/// which further describe them.
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Property {
	pub id: Uuid,
	pub t: PropertyVariant,
	pub name: String,
}

impl Property {
	pub fn new(name: &str, t: PropertyVariant) -> Result<Self, Error> {
		Ok(Property {
			id: Uuid::new_v4(),
			name: name.to_string(),
			t,
		})
	}
}

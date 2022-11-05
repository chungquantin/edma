use uuid::Uuid;

use crate::Error;

pub enum PropertyVariant {
	Unknown,
	String,
	UInt32,
	UInt64,
	UInt128,
	Document,
	VecString,
	VecUint32,
	VecUint64,
	VecUint128,
}

impl Default for PropertyVariant {
	fn default() -> Self {
		PropertyVariant::Unknown
	}
}

/// ## Property
/// Nodes and relationships can have properties (key-value pairs),
/// which further describe them.
#[derive(Default)]
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

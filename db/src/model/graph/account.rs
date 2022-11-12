use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Eq, Debug, Clone)]
pub enum AccountDiscriminator {
	None = 0,
	Vertex = 1,
	Label = 2,
	Property = 3,
	Relationship = 4,
}

impl AccountDiscriminator {
	pub fn serialize(&self) -> Vec<u8> {
		bincode::serialize(self).unwrap()
	}
}

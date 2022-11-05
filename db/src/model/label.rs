use uuid::Uuid;

use crate::Error;

/// ## Label
/// Nodes can have zero or more labels to define (classify) what kind of nodes they are.
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Label {
	pub name: String,
	pub id: Uuid,
}

impl Drop for Label {
	fn drop(&mut self) {
		self.name = String::default();
		self.id = Uuid::default();
	}
}

impl Label {
	pub fn new(name: &str) -> Result<Self, Error> {
		Ok(Label {
			name: name.to_string(),
			id: Uuid::new_v4(),
		})
	}
}

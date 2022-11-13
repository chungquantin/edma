use uuid::Uuid;

use crate::{
	util::{build_bytes, build_meta, Component},
	AccountDiscriminator, Error,
};

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

	pub fn serialize(label: &Label) -> Result<Vec<u8>, Error> {
		let discriminator = AccountDiscriminator::Label.serialize();
		let meta = &build_meta(1, label.name.len());
		let val = [discriminator, meta.to_vec(), label.name.as_bytes().to_vec()].concat();

		Ok(val)
	}

	pub fn multi_serialize(labels: &Vec<Label>) -> Result<Vec<u8>, Error> {
		let ll = labels.len() as u8; // Label list length
		let uuid_len = Component::Uuid(Uuid::nil()).len();

		let label_components =
			labels.iter().map(|l| Component::Uuid(l.id)).collect::<Vec<Component>>();
		// Handle byte concatenate for label components
		let label_discriminator = AccountDiscriminator::Label.serialize();
		let _labels = build_bytes(&label_components).unwrap();
		let label_meta = &build_meta(ll, uuid_len);
		// (Label discriminator, Label byte array, Label metadata)
		let (l_dis, l, l_meta) =
			(label_discriminator.as_slice(), label_meta.as_slice(), _labels.as_slice());
		let labels_concat = [l_dis, l, l_meta].concat();

		Ok(labels_concat)
	}
}

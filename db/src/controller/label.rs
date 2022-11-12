use crate::util::{
	build_bytes, build_meta, deserialize_data_with_meta, from_uuid_bytes, Component,
};
use crate::{AccountDiscriminator, DatastoreAdapter, Error, Label, SimpleTransaction};

impl_controller!(LabelController("labels:v1"));

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for LabelController {
	fn default() -> Self {
		LabelController::new().unwrap()
	}
}

impl LabelController {
	/// # Create a new vertex from labels and properties
	pub async fn create_label(&self, name: &str) -> Result<Label, Error> {
		let label = Label::new(name).unwrap();
		let mut tx = self.config.ds.transaction(true).unwrap();

		let cf = self.get_cf();
		let key = build_bytes(&[Component::Uuid(label.id)]).unwrap();
		let discriminator = AccountDiscriminator::Label.serialize();
		let meta = &build_meta(1, name.len());
		let val = [discriminator, meta.to_vec(), name.as_bytes().to_vec()].concat();

		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(label)
	}

	pub async fn create_labels(&self, names: Vec<&str>) -> Result<Vec<Label>, Error> {
		let mut result = Vec::<Label>::new();
		for name in names.iter() {
			let label = self.create_label(name).await.unwrap();
			result.push(label);
		}

		Ok(result)
	}

	pub async fn get_label(&self, id: Vec<u8>) -> Result<Label, Error> {
		let tx = self.config.ds.transaction(false).unwrap();

		let cf = self.get_cf();
		let raw = tx.get(cf, id.to_vec()).await.unwrap();
		match raw {
			Some(r) => {
				let (data, _, _) = deserialize_data_with_meta(r, true).unwrap();
				Ok(Label {
					id: from_uuid_bytes(&id).unwrap(),
					name: String::from_utf8(data[0].to_vec()).unwrap(),
				})
			}
			None => panic!("No label value"),
		}
	}
}

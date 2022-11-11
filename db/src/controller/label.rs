use crate::util::{build_bytes, from_uuid_bytes, Component};
use crate::{DatastoreAdapter, Error, Label, SimpleTransaction};

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
		let val = name.as_bytes();

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
		let val = tx.get(cf, id.to_vec()).await.unwrap();
		match val {
			Some(v) => Ok(Label {
				id: from_uuid_bytes(&id).unwrap(),
				name: String::from_utf8(v).unwrap(),
			}),
			None => panic!("No label value"),
		}
	}
}

// #[cfg(feature = "test-suite")]
#[cfg(test)]
#[tokio::test]
async fn should_create_label() {
	let lc = LabelController::default();
	let res = lc.create_label("Person").await.unwrap();
	let label = lc.get_label(res.id.as_bytes().to_vec()).await.unwrap();
	assert_eq!(label, res);
}

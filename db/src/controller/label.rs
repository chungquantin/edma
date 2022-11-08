use crate::mac::Controller;
use crate::model::adapter::DatastoreAdapter;
use crate::util::{build_bytes, from_uuid_bytes, Component};
use crate::{storage::DatastoreManager, Error, RocksDBAdapter};
use crate::{Label, SimpleTransaction};

impl_controller!(get RocksDBAdapter; from rocks_db for LabelController);

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for LabelController {
	fn default() -> Self {
		LabelController::new(DatastoreManager::RocksDBAdapter).unwrap()
	}
}

impl LabelController {
	fn get_cf(&self) -> Option<Vec<u8>> {
		Some(self.cf.into())
	}

	/// # Create a new vertex from labels and properties
	pub async fn create_label(&self, name: &str) -> Result<(), Error> {
		let label = Label::new(name).unwrap();
		let mut tx = self.ds.transaction(true).unwrap();

		let cf = self.get_cf();
		let key = build_bytes(&[Component::Uuid(label.id)]).unwrap();
		let val = name;

		tx.set(cf, key, val).await.unwrap();
		Ok(())
	}

	pub async fn get_label(&self, id: Vec<u8>) -> Result<Label, Error> {
		let tx = self.ds.transaction(false).unwrap();

		let cf = self.get_cf();
		let val = tx.get(cf, id.to_vec()).await.unwrap().unwrap();
		Ok(Label {
			id: from_uuid_bytes(&id).unwrap(),
			name: String::from_utf8(val).unwrap(),
		})
	}
}

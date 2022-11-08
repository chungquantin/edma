use std::collections::HashSet;

use crate::mac::Controller;
use crate::model::adapter::DatastoreAdapter;
use crate::util::{build_bytes, from_uuid_bytes, Component};
use crate::SimpleTransaction;
use crate::{storage::DatastoreManager, Error, RocksDBAdapter, Vertex};

use uuid::Uuid;

impl_controller!(get RocksDBAdapter; from rocks_db for VertexController);

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for VertexController {
	fn default() -> Self {
		VertexController::new(DatastoreManager::RocksDBAdapter).unwrap()
	}
}

impl VertexController {
	fn get_cf(&self) -> Option<Vec<u8>> {
		Some(self.cf.into())
	}

	/// # Create a new vertex from labels and properties
	pub async fn create_vertex(
		&self,
		labels: Vec<Uuid>,
		props: HashSet<Uuid, Vec<u8>>,
	) -> Result<(), Error> {
		let v = Vertex::new(labels, props).unwrap();
		let mut tx = self.ds.transaction(true).unwrap();
		let cf = self.get_cf();

		let key = build_bytes(&[Component::Uuid(v.id)]).unwrap();
		let labels = v.labels.iter().map(|l| Component::Uuid(*l)).collect::<Vec<Component>>();
		let val = build_bytes(&labels).unwrap();

		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(())
	}

	pub async fn get_vertex(&self, id: Vec<u8>) -> Result<(), Error> {
		let cf = self.get_cf();
		let mut tx = self.ds.transaction(false).unwrap();

		let mut values = tx.get(cf, id).await.unwrap().unwrap();
		let label_ids = from_uuid_bytes(&mut values).unwrap();
		Ok(())
	}
}

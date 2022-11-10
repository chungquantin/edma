use std::collections::HashMap;

use crate::model::adapter::DatastoreAdapter;
use crate::util::{build_bytes, from_uuid_bytes, from_vec_uuid_bytes, Component};
use crate::{Error, SimpleTransaction, Vertex};

use uuid::Uuid;

impl_controller!(VertexController("vertices:v1"));

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for VertexController {
	fn default() -> Self {
		VertexController::new().unwrap()
	}
}

impl VertexController {
	/// # Create a new vertex from labels and properties
	pub async fn create_vertex(
		&self,
		labels: Vec<Uuid>,
		props: HashMap<Uuid, Vec<u8>>,
	) -> Result<Vertex, Error> {
		let v = Vertex::new(labels, props).unwrap();
		let mut tx = self.config.ds.transaction(true).unwrap();
		let cf = self.get_cf();

		let key = build_bytes(&[Component::Uuid(v.id)]).unwrap();
		let labels = v.labels.iter().map(|l| Component::Uuid(*l)).collect::<Vec<Component>>();
		let val = build_bytes(&labels).unwrap();
		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(v)
	}

	pub async fn get_vertex(&self, id: Vec<u8>) -> Result<Vertex, Error> {
		let cf = self.get_cf();
		let tx = self.config.ds.transaction(false).unwrap();

		let value = tx.get(cf, id.to_vec()).await.unwrap();

		match value {
			Some(mut v) => {
				let uuid = from_uuid_bytes(&id).unwrap();
				let label_ids = from_vec_uuid_bytes(&mut v).unwrap();

				return Ok(Vertex {
					id: uuid,
					labels: label_ids,
					props: HashMap::default(),
				});
			}
			None => panic!("No vertex found"),
		}
	}
}

// #[cfg(feature = "test-suite")]
#[cfg(test)]
#[tokio::test]
async fn should_create_label() {
	let vc = VertexController::default();
	let res = vc.create_vertex(vec![], HashMap::default()).await.unwrap();
	let vertex = vc.get_vertex(res.id.as_bytes().to_vec()).await.unwrap();
	assert_eq!(vertex, res);
}

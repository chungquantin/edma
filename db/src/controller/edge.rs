use serde_json::Value;
use uuid::Uuid;

use crate::util::{build_bytes, Component};
use crate::{Edge, EdgePropertyController, Error, Identifier, SimpleTransaction};

impl_controller!(EdgeController("edges:v1"));

impl<'a> EdgeController<'a> {
	fn key(&self, out_id: Uuid, t: &Identifier, in_id: Uuid) -> Vec<u8> {
		build_bytes(&[Component::Uuid(out_id), Component::Identifier(t), Component::Uuid(in_id)])
			.unwrap()
	}

	pub async fn create(
		&self,
		in_id: Uuid,
		t: &str,
		out_id: Uuid,
		props: Value,
	) -> Result<Edge, Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();

		let cf = self.get_cf();
		let epc = EdgePropertyController::new(self.ds_ref);
		let props = epc.create(in_id, t, out_id, props).await.unwrap();
		let t = &Identifier::new(t).unwrap();
		let key = self.key(in_id, t, out_id);

		tx.set(cf, key, []).await.unwrap();
		tx.commit().await.unwrap();

		let edge = Edge::new(in_id, t.clone(), out_id, props).unwrap();
		Ok(edge)
	}

	pub async fn delete(&self, in_id: Uuid, t: &str, out_id: Uuid) -> Result<(), Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		let cf = self.get_cf();
		let t = &Identifier::new(t).unwrap();
		let key = self.key(in_id, t, out_id);
		tx.del(cf, key).await.unwrap();
		tx.commit().await
	}

	pub async fn get(&self, in_id: Uuid, t: &str, out_id: Uuid) -> Result<Edge, Error> {
		let epc = EdgePropertyController::new(self.ds_ref);
		let props = epc.iterate_from_edge(in_id, t, out_id).await.unwrap();
		let t = Identifier::new(t).unwrap();
		let edge = Edge::new(in_id, t, out_id, props).unwrap();
		Ok(edge)
	}
}

use std::collections::HashMap;

use uuid::Uuid;

use crate::util::{build_bytes, from_uuid_bytes, Component};
use crate::{DatastoreAdapter, Error, Identifier, Property, Relationship, SimpleTransaction};

impl_controller!(RelationshipController("relationships:v1"));

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for RelationshipController {
	fn default() -> Self {
		RelationshipController::new().unwrap()
	}
}

impl RelationshipController {
	pub async fn create_property(
		&self,
		s_node: Uuid,
		t_node: Uuid,
		t: &str,
		props: HashMap<Uuid, Vec<u8>>,
	) -> Result<Relationship, Error> {
		let mut tx = self.config.ds.transaction(true).unwrap();

		let cf = self.get_cf();
		let t_id = Identifier::new(t.to_string()).unwrap();
		let key = build_bytes(&[
			Component::Uuid(s_node),
			Component::Uuid(s_node),
			Component::Identifier(&t_id),
			Component::Uuid(t_node),
		])
		.unwrap();
		let relationship = Relationship::new(s_node, t_node, t_id, props).unwrap();
		let val = [];

		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(relationship)
	}
}

use std::collections::HashMap;
use uuid::Uuid;

use crate::util::{build_bytes, deserialize_data_with_meta, from_uuid_bytes, Component};
use crate::{
	AccountDiscriminator, DatastoreAdapter, Error, Identifier, Relationship, SimpleTransaction,
};

impl_controller!(RelationshipController("relationships:v1"));

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for RelationshipController {
	fn default() -> Self {
		RelationshipController::new().unwrap()
	}
}

impl RelationshipController {
	pub async fn create_relationship(
		&self,
		source_vertex: Uuid,
		target_vertex: Uuid,
		t: &str,
		props: HashMap<Uuid, Vec<u8>>,
	) -> Result<Relationship, Error> {
		let mut tx = self.config.ds.transaction(true).unwrap();
		let prop_components =
			props.iter().map(|(id, val)| Component::Property(id, val)).collect::<Vec<Component>>();

		let cf = self.get_cf();
		let t_id = Identifier::new(t.to_string()).unwrap();
		let key = build_bytes(&[
			Component::Uuid(source_vertex),
			Component::Identifier(&t_id),
			Component::Uuid(target_vertex),
		])
		.unwrap();

		// Handle byte concatenate for property components
		let _prop_discriminator = AccountDiscriminator::Property.serialize();
		let _props = &build_bytes(&prop_components).unwrap();
		// (Property discriminator, Property byte array - meta for each property generated)
		let (p_dis, p) = (_prop_discriminator.as_slice(), _props.as_slice());
		let props_concat = [p_dis, p].concat();

		let val = [props_concat].concat();

		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();

		let relationship = Relationship::new(source_vertex, target_vertex, t_id, props).unwrap();
		Ok(relationship)
	}

	pub async fn get_relationship(
		&self,
		source_vertex: Uuid,
		target_vertex: Uuid,
		t: &str,
	) -> Result<Relationship, Error> {
		let tx = self.config.ds.transaction(false).unwrap();
		let cf = self.get_cf();
		let t_id = Identifier::new(t.to_string()).unwrap();
		let key = build_bytes(&[
			Component::Uuid(source_vertex),
			Component::Identifier(&t_id),
			Component::Uuid(target_vertex),
		])
		.unwrap();

		let mut props = HashMap::default();
		let raw_relationship = tx.get(cf, key).await.unwrap();
		match raw_relationship {
			Some(r) => {
				let (props_bytes, _, _) = deserialize_data_with_meta(r, true).unwrap();
				let uuid_len = Component::Uuid(Uuid::nil()).len();
				for slice in props_bytes {
					let (uuid, value) = (&slice[..uuid_len], &slice[uuid_len..]);
					props.insert(from_uuid_bytes(uuid).unwrap(), value.to_vec());
				}
			}
			None => panic!("No relationship found"),
		}

		let relationship = Relationship::new(source_vertex, target_vertex, t_id, props).unwrap();
		Ok(relationship)
	}
}

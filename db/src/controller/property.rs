use crate::util::{build_bytes, from_uuid_bytes, Component};
use crate::{DatastoreAdapter, Error, PropType, Property, SimpleTransaction};

impl_controller!(PropertyController("properties:v1"));

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for PropertyController {
	fn default() -> Self {
		PropertyController::new().unwrap()
	}
}

impl PropertyController {
	pub async fn create_property(&self, name: &str, variant: PropType) -> Result<Property, Error> {
		let mut tx = self.config.ds.transaction(true).unwrap();

		let cf = self.get_cf();
		let property = Property::new(name, variant).unwrap();
		let key = build_bytes(&[Component::Uuid(property.id)]).unwrap();
		let val = Property::serialize(&property).unwrap();

		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(property)
	}

	pub async fn create_properties(
		&self,
		properties: Vec<(&str, PropType)>,
	) -> Result<Vec<Property>, Error> {
		let mut result = vec![];
		let mut tx = self.config.ds.transaction(true).unwrap();

		for (name, variant) in properties {
			let cf = self.get_cf();
			let property = Property::new(name, variant).unwrap();
			let key = build_bytes(&[Component::Uuid(property.id)]).unwrap();
			let val = Property::serialize(&property).unwrap();

			tx.set(cf, key, val).await.unwrap();
			result.push(property);
		}

		tx.commit().await.unwrap();
		Ok(result)
	}

	pub async fn get_property(&self, id: Vec<u8>) -> Result<Property, Error> {
		let tx = self.config.ds.transaction(false).unwrap();

		let cf = self.get_cf();
		let val = tx.get(cf, id.to_vec()).await.unwrap();
		match val {
			Some(v) => {
				let (name, t) = Property::deserialize(v).unwrap();
				Ok(Property {
					id: from_uuid_bytes(&id).unwrap(),
					name,
					t,
				})
			}
			None => panic!("No label value"),
		}
	}
}

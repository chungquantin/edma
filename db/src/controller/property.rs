use crate::util::{build_bytes, from_uuid_bytes, Component};
use crate::{DatastoreAdapter, Error, Property, PropertyVariant, SimpleTransaction};

impl_controller!(PropertyController("properties:v1"));

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for PropertyController {
	fn default() -> Self {
		PropertyController::new().unwrap()
	}
}

impl PropertyController {
	pub async fn create_property(
		&self,
		name: &str,
		variant: PropertyVariant,
	) -> Result<Property, Error> {
		let mut tx = self.config.ds.transaction(true).unwrap();

		let cf = self.get_cf();
		// First four bytes are the property
		let serialized_variant = bincode::serialize::<PropertyVariant>(&variant).unwrap();
		let property = Property::new(name, variant).unwrap();
		let key = build_bytes(&[Component::Uuid(property.id)]).unwrap();
		// Dynamic length string will be concatenated at the end
		let val = [serialized_variant, name.as_bytes().to_vec()].concat();

		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(property)
	}

	pub async fn get_property(&self, id: Vec<u8>) -> Result<Property, Error> {
		let tx = self.config.ds.transaction(false).unwrap();

		let cf = self.get_cf();
		let val = tx.get(cf, id.to_vec()).await.unwrap();
		match val {
			Some(v) => {
				let property = &v[0..4];
				let name = &v[4..];

				return Ok(Property {
					id: from_uuid_bytes(&id).unwrap(),
					name: String::from_utf8(name.to_vec()).unwrap(),
					t: bincode::deserialize::<PropertyVariant>(property).unwrap(),
				});
			}
			None => panic!("No label value"),
		}
	}
}

#[cfg(test)]
#[tokio::test]
async fn should_create_property() {
	let pc = PropertyController::default();
	let res = pc.create_property("Name", PropertyVariant::String).await.unwrap();
	let property = pc.get_property(res.id.as_bytes().to_vec()).await.unwrap();
	assert_eq!(property, res);
}

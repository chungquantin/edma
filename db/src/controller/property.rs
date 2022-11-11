use crate::util::{build_bytes, build_offset, deserialize_full_data, from_uuid_bytes, Component};
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
		let property_offset = &build_offset(1, serialized_variant.len());
		let name = name.as_bytes();
		let name_offset = &build_offset(1, name.len());
		// Dynamic length string will be concatenated at the end
		let val = [property_offset, &serialized_variant, name_offset, name].concat();

		let key = build_bytes(&[Component::Uuid(property.id)]).unwrap();
		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(property)
	}

	pub async fn create_properties(
		&self,
		properties: Vec<(&str, PropertyVariant)>,
	) -> Result<Vec<Property>, Error> {
		let mut result = vec![];
		for (name, variant) in properties {
			let property = self.create_property(name, variant).await.unwrap();
			result.push(property);
		}
		Ok(result)
	}

	pub async fn get_property(&self, id: Vec<u8>) -> Result<Property, Error> {
		let tx = self.config.ds.transaction(false).unwrap();

		let cf = self.get_cf();
		let val = tx.get(cf, id.to_vec()).await.unwrap();
		match val {
			Some(v) => {
				let deserialized = deserialize_full_data(v, false).unwrap();
				let property = &deserialized[0].first().unwrap();
				let name = &deserialized[1].first().unwrap();

				let name = String::from_utf8(name.to_vec()).unwrap();
				let t = bincode::deserialize::<PropertyVariant>(property).unwrap();
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

#[cfg(test)]
#[tokio::test]
async fn should_create_property() {
	let pc = PropertyController::default();
	let res = pc.create_property("Name", PropertyVariant::String).await.unwrap();
	let property = pc.get_property(res.id.as_bytes().to_vec()).await.unwrap();
	assert_eq!(property, res);
}

#[cfg(test)]
#[tokio::test]
async fn should_create_properties() {
	let pc = PropertyController::default();
	let properties = pc
		.create_properties(vec![
			("name", PropertyVariant::String),
			("age", PropertyVariant::UInt128),
		])
		.await
		.unwrap();
	assert_eq!(properties.len(), 2);
}

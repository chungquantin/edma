use std::collections::HashMap;

use crate::{
	model::adapter::DatastoreAdapter,
	util::{build_bytes, deserialize_byte_data, from_uuid_bytes, Component},
	AccountDiscriminator, Error, Label, Property, SimpleTransaction, Vertex,
};

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
	/// ## Description
	/// Because Vertex has multiple dynamic sized attributes:
	/// - labels: Vec<Label>
	/// - props: HashMap<Uuid, Vec<u8>>
	/// It will be a bit more complicated.
	///
	/// Data will be store in Vertex as
	/// + label_discriminator | label_meta | label data
	pub async fn create_vertex(
		&self,
		labels: Vec<Label>,
		props: HashMap<Uuid, Vec<u8>>,
	) -> Result<Vertex, Error> {
		let mut tx = self.config.ds.transaction(true).unwrap();
		let cf = self.get_cf();

		let labels_bytes = Label::multi_serialize(&labels).unwrap();
		let properties_bytes = Property::multi_serialize(&props).unwrap();

		let v = Vertex::new(labels.to_vec(), props).unwrap();
		let key = build_bytes(&[Component::Uuid(v.id)]).unwrap();

		let val = [labels_bytes, properties_bytes].concat();
		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(v)
	}

	/// # Get single vertex from datastore
	pub async fn get_vertex(&self, id: Vec<u8>) -> Result<Vertex, Error> {
		let cf = self.get_cf();
		let tx = self.config.ds.transaction(false).unwrap();

		let value = tx.get(cf, id.to_vec()).await.unwrap();

		match value {
			Some(v) => {
				let uuid = from_uuid_bytes(&id).unwrap();
				let deserialized = deserialize_byte_data(v.to_vec(), true).unwrap();
				let mut properties = HashMap::<Uuid, Vec<u8>>::new();
				let mut label_ids = Vec::<Uuid>::new();

				for (data, raw_discriminator) in deserialized.iter() {
					match bincode::deserialize::<AccountDiscriminator>(raw_discriminator) {
						Ok(discriminator) => match discriminator {
							AccountDiscriminator::Property => {
								let uuid_len = Component::Uuid(Uuid::nil()).len();
								for slice in data {
									let (uuid, value) = (&slice[..uuid_len], &slice[uuid_len..]);
									properties
										.insert(from_uuid_bytes(uuid).unwrap(), value.to_vec());
								}
							}
							AccountDiscriminator::Label => {
								for label in data {
									label_ids.push(from_uuid_bytes(label).unwrap());
								}
							}
							_ => unreachable!(),
						},
						_ => panic!("No match discriminator found"),
					}
				}

				Ok(Vertex {
					id: uuid,
					labels: label_ids,
					props: properties,
				})
			}
			None => panic!("No vertex found"),
		}
	}

	/// # Get multiple vertices from datastore
	pub async fn get_vertices(&self, ids: Vec<Vec<u8>>) -> Result<Vec<Vertex>, Error> {
		let mut vertices = Vec::<Vertex>::new();

		for id in ids.iter() {
			let vertex = self.get_vertex(id.to_vec()).await.unwrap();
			vertices.push(vertex);
		}

		Ok(vertices)
	}
}

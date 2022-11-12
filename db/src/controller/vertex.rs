use std::collections::HashMap;

use crate::{
	model::adapter::DatastoreAdapter,
	util::{build_bytes, build_meta, deserialize_byte_data, from_uuid_bytes, Component},
	AccountDiscriminator, Error, Label, SimpleTransaction, Vertex,
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
		let uuid_len = Component::Uuid(Uuid::nil()).len();
		let ll = labels.len() as u8; // Label list length

		let mut tx = self.config.ds.transaction(true).unwrap();
		let cf = self.get_cf();

		let label_components =
			labels.iter().map(|l| Component::Uuid(l.id)).collect::<Vec<Component>>();
		let prop_components =
			props.iter().map(|(id, val)| Component::Property(id, val)).collect::<Vec<Component>>();

		// Handle byte concatenate for label components
		let label_discriminator = AccountDiscriminator::Label.serialize();
		let _labels = build_bytes(&label_components).unwrap();
		let label_meta = &build_meta(ll, uuid_len);
		// (Label discriminator, Label byte array, Label metadata)
		let (l_dis, l, l_meta) =
			(label_discriminator.as_slice(), label_meta.as_slice(), _labels.as_slice());
		let labels_concat = [l_dis, l, l_meta].concat();

		// Handle byte concatenate for property components
		let _prop_discriminator = AccountDiscriminator::Property.serialize();
		let _props = &build_bytes(&prop_components).unwrap();
		// (Property discriminator, Property byte array - meta for each property generated)
		let (p_dis, p) = (_prop_discriminator.as_slice(), _props.as_slice());
		let props_concat = [p_dis, p].concat();

		let v = Vertex::new(labels.to_vec(), props).unwrap();
		let key = build_bytes(&[Component::Uuid(v.id)]).unwrap();

		let val = [labels_concat, props_concat].concat();
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

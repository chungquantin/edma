use crate::{
	interface::KeyValuePair,
	util::{build_bytes, deserialize_byte_data, from_uuid_bytes, Component},
	vertex_property::VertexPropertyController,
	AccountDiscriminator, Error, Label, SimpleTransaction, Vertex,
};

use serde_json::Value;
use uuid::Uuid;

impl_controller!(VertexController("vertices:v1"));

impl<'a> VertexController<'a> {
	pub fn key(&self, id: Uuid) -> Vec<u8> {
		build_bytes(&[Component::Uuid(id)]).unwrap()
	}

	/// # Create a new vertex from labels and properties
	/// ## Description
	/// Because Vertex has multiple dynamic sized attributes:
	/// - labels: Vec<Label>
	/// - props: serde_json::Value
	/// It will be a bit more complicated.
	///
	/// Data will be store in Vertex as
	/// + label_discriminator | label_meta | label data
	pub async fn create(&self, labels: Vec<Label>, props: Value) -> Result<Vertex, Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		let cf = self.get_cf();

		let labels_bytes = Label::multi_serialize(&labels).unwrap();
		let vpc = VertexPropertyController::new(self.ds_ref);
		let mut v = Vertex::new(labels.to_vec()).unwrap();
		let props = vpc.create(v.id, props).await.unwrap();
		v.add_props(props).unwrap();

		let key = self.key(v.id);

		let val = [labels_bytes].concat();
		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(v)
	}

	async fn deserialize(&self, id: Vec<u8>, v: Vec<u8>) -> Result<Vertex, Error> {
		let uuid = from_uuid_bytes(&id).unwrap();
		let deserialized = deserialize_byte_data(v.to_vec(), true).unwrap();
		let mut label_ids = Vec::<Uuid>::new();

		for (data, raw_discriminator) in deserialized.iter() {
			match bincode::deserialize::<AccountDiscriminator>(raw_discriminator) {
				Ok(discriminator) => match discriminator {
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

		let vpc = VertexPropertyController::new(self.ds_ref);
		let props = vpc.iterate_from_vertex(id).await.unwrap();
		Ok(Vertex {
			id: uuid,
			labels: label_ids,
			props,
		})
	}

	pub async fn delete(&self, id: Vec<u8>) -> Result<(), Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		let cf = self.get_cf();
		let key = id;
		tx.del(cf, key).await.unwrap();
		tx.commit().await
	}

	/// # Get single vertex from datastore
	pub async fn get(&self, id: Vec<u8>) -> Result<Vertex, Error> {
		let cf = self.get_cf();

		let tx = self.get_ds().transaction(false).unwrap();
		let value = tx.get(cf, id.to_vec()).await.unwrap();
		match value {
			Some(v) => self.deserialize(id, v).await,
			None => panic!("No vertex found"),
		}
	}

	async fn iterate(
		&self,
		iterator: Vec<Result<KeyValuePair, Error>>,
	) -> Result<Vec<Vertex>, Error> {
		let mut result: Vec<Vertex> = vec![];
		for pair in iterator.iter() {
			let (k, v) = pair.as_ref().unwrap();
			let vertex = self.deserialize(k.to_vec(), v.to_vec()).await.unwrap();
			result.push(vertex);
		}

		Ok(result)
	}

	/// # Get multiple vertices from datastore
	pub async fn multi_get(&self, ids: Vec<Vec<u8>>) -> Result<Vec<Vertex>, Error> {
		let mut vertices = Vec::<Vertex>::new();

		for id in ids.iter() {
			let vertex = self.get(id.to_vec()).await.unwrap();
			vertices.push(vertex);
		}

		Ok(vertices)
	}

	pub async fn iterate_all(&self) -> Result<Vec<Vertex>, Error> {
		let tx = self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();

		let iterator = tx.iterate(cf).await.unwrap();
		self.iterate(iterator).await
	}
}

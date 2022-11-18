use crate::{
	interface::KeyValuePair,
	util::{build_bytes, deserialize_byte_data, from_uuid_bytes, Component},
	vertex_property::VertexPropertyRepository,
	AccountDiscriminator, Error, Label, SimpleTransaction, Vertex,
};

use gremlin::GValue;
use uuid::Uuid;

impl_repository!(VertexRepository("vertices:v1"));

impl<'a> VertexRepository<'a> {
	pub fn key(&self, id: Uuid) -> Vec<u8> {
		build_bytes(&[Component::Uuid(id)]).unwrap()
	}

	pub async fn v(&self, ids: &Vec<GValue>) -> Result<Vec<Option<Vertex>>, Error> {
		let tx = self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();
		match ids.first() {
			Some(id) => {
				let value = id.get::<Uuid>().unwrap();
				let key = self.key(*value);

				let vertex = tx.get(cf, key.to_vec()).await.unwrap();
				Ok(vec![match vertex {
					Some(v) => Some(self.from_pair(&(key, v)).await.unwrap()),
					_ => None,
				}])
			}
			_ => self.iterate_all().await,
		}
	}

	/// # Create a new vertex from labels and properties
	pub async fn add_v(&self, labels: &Vec<GValue>) -> Result<Vertex, Error> {
		let mut serialized_labels = Vec::<Label>::new();
		for label in labels.iter() {
			let val = label.get::<String>().unwrap();
			let l = Label::new(val).unwrap();
			serialized_labels.push(l);
		}

		let bytes = Label::multi_serialize(&serialized_labels).unwrap();
		let v = Vertex::new(serialized_labels).unwrap();
		let cf = self.get_cf();
		let key = self.key(v.id);
		let val = [bytes].concat();

		let mut tx = self.get_ds().transaction(true).unwrap();
		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(v)
	}

	async fn from_pair(&self, p: &KeyValuePair) -> Result<Vertex, Error> {
		let (k, v) = p;
		let uuid = from_uuid_bytes(k).unwrap();
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

		let vpc = VertexPropertyRepository::new(self.ds_ref);
		let props = vpc.iterate_from_vertex(k.to_vec()).await.unwrap();
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

	async fn iterate(
		&self,
		iterator: Vec<Result<KeyValuePair, Error>>,
	) -> Result<Vec<Option<Vertex>>, Error> {
		let mut result: Vec<Option<Vertex>> = vec![];
		for pair in iterator.iter() {
			let p_ref = pair.as_ref().unwrap();
			let vertex = self.from_pair(p_ref).await.unwrap();
			result.push(Some(vertex));
		}

		Ok(result)
	}

	pub async fn iterate_all(&self) -> Result<Vec<Option<Vertex>>, Error> {
		let tx = self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();

		let iterator = tx.iterate(cf).await.unwrap();
		self.iterate(iterator).await
	}
}

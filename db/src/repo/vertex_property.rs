use gremlin::GValue;
use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::interface::KeyValuePair;
use crate::util::{build_bytes, build_json_value, Component};
use crate::{Error, SimpleTransaction};

impl_repository!(VertexPropertyRepository("vertex-properties:v1"));

impl<'a> VertexPropertyRepository<'a> {
	pub fn key(&self, vertex_id: Uuid, k: &String) -> Result<Vec<u8>, Error> {
		Ok(build_bytes(&[Component::Uuid(vertex_id), Component::Bytes(k.as_bytes())]).unwrap())
	}

	// # Create a new vertex property
	// (key, field, value)
	pub async fn property(
		&self,
		vertex_id: Uuid,
		field: &str,
		value: GValue,
	) -> Result<(Vec<u8>, String, GValue), Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		let cf = self.get_cf();
		let val = build_bytes(&[Component::GValueType(&value), Component::GValue(&value)]).unwrap();
		let prop = value.get::<String>().unwrap();
		let key = self.key(vertex_id, prop).unwrap();
		tx.set(cf, key.to_vec(), val).await.unwrap();
		tx.commit().await.unwrap();
		Ok((key, field.to_string(), value))
	}

	fn iterate(&self, iterator: Vec<Result<KeyValuePair, Error>>) -> Result<Value, Error> {
		let uuid_len = Component::Uuid(Uuid::nil()).len();
		let mut map: Map<String, Value> = Map::default();
		iterator.iter().for_each(|p| {
			let (k, v) = p.as_ref().unwrap();
			let attr = String::from_utf8(k[uuid_len..].to_vec()).unwrap();
			let value = build_json_value(v.to_vec()).unwrap();
			map.insert(attr, value);
		});

		Ok(json!(map))
	}

	pub async fn iterate_from_vertex(&self, vertex_id: Vec<u8>) -> Result<Value, Error> {
		let tx = &self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();
		let prefix = build_bytes(&[Component::Bytes(&vertex_id)]).unwrap();
		let iterator = tx.prefix_iterate(cf, prefix).await.unwrap();
		self.iterate(iterator)
	}
}

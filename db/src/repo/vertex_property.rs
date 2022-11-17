use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::interface::KeyValuePair;
use crate::util::{build_bytes, build_json_value, Component};
use crate::{Error, SimpleTransaction};

impl_controller!(VertexPropertyRepository("vertex-properties:v1"));

impl<'a> VertexPropertyRepository<'a> {
	pub fn key(&self, vertex_id: Uuid, k: &String) -> Result<Vec<u8>, Error> {
		Ok(build_bytes(&[Component::Uuid(vertex_id), Component::Bytes(k.as_bytes())]).unwrap())
	}

	pub async fn create(&self, vertex_id: Uuid, data: Value) -> Result<Value, Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		if !data.is_object() {
			panic!("Data type must be object");
		}
		let o = data.as_object().unwrap();
		for k in o.keys() {
			let cf = self.get_cf();
			let val = o.get(k).unwrap();
			let json_value =
				build_bytes(&[Component::JsonValueType(val), Component::JsonValue(val)]).unwrap();
			let key = self.key(vertex_id, k).unwrap();
			tx.put(cf, key, json_value).await.unwrap();
		}
		tx.commit().await.unwrap();
		Ok(data)
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

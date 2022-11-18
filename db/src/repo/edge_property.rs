use gremlin::GValue;
use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::interface::KeyValuePair;
use crate::util::build_json_value;
use crate::util::{build_bytes, Component};
use crate::{Error, Identifier, SimpleTransaction};

impl_repository!(EdgePropertyRepository("edge-properties:v1"));

impl<'a> EdgePropertyRepository<'a> {
	fn edge_key(&self, in_id: Uuid, t: &Identifier, out_id: Uuid) -> Vec<u8> {
		build_bytes(&[Component::Uuid(in_id), Component::Identifier(t), Component::Uuid(out_id)])
			.unwrap()
	}

	fn key(&self, in_id: Uuid, t: &Identifier, out_id: Uuid, k: &String) -> Vec<u8> {
		let edge_key = self.edge_key(in_id, t, out_id);
		build_bytes(&[Component::Bytes(&edge_key), Component::Bytes(k.as_bytes())]).unwrap()
	}

	pub async fn get_value(
		&self,
		in_id: Uuid,
		t: &str,
		out_id: Uuid,
		k: &String,
	) -> Result<Value, Error> {
		let tx = self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();
		let t_id = Identifier::new(t).unwrap();
		let key = self.key(in_id, &t_id, out_id, k);
		let value = tx.get(cf, key).await.unwrap();
		match value {
			Some(v) => Ok(build_json_value(v.to_vec()).unwrap()),
			_ => panic!("Not match key found"),
		}
	}

	// # Create a new property
	// (key, field, value)
	pub async fn property(
		&self,
		in_id: Uuid,
		t: &str,
		out_id: Uuid,
		field: &str,
		value: GValue,
	) -> Result<(Vec<u8>, String, GValue), Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		let cf = self.get_cf();
		let val = build_bytes(&[Component::GValueType(&value), Component::GValue(&value)]).unwrap();
		let t_id = &Identifier::new(t).unwrap();
		let key = self.key(in_id, t_id, out_id, &field.to_string());
		tx.set(cf, key.to_vec(), val).await.unwrap();
		tx.commit().await.unwrap();
		Ok((key, field.to_string(), value))
	}

	pub async fn delete(&self, in_id: Uuid, t: &Identifier, out_id: Uuid) -> Result<(), Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		let cf = self.get_cf().unwrap();
		let edge_key = self.edge_key(in_id, t, out_id);
		let iterator = tx.prefix_iterate(Some(cf.to_vec()), edge_key).await.unwrap();
		for p in iterator.iter() {
			let (k, _) = p.as_ref().unwrap();
			tx.del(Some(cf.to_vec()), &**k).await.unwrap();
		}
		tx.commit().await
	}

	fn iterate(
		&self,
		offset: usize,
		iterator: Vec<Result<KeyValuePair, Error>>,
	) -> Result<Value, Error> {
		let mut map: Map<String, Value> = Map::default();
		iterator.iter().for_each(|p| {
			let (k, v) = p.as_ref().unwrap();
			let attr = String::from_utf8(k[offset..].to_vec()).unwrap();
			let value = build_json_value(v.to_vec()).unwrap();
			map.insert(attr, value);
		});

		Ok(json!(map))
	}

	pub async fn iterate_for_all(&'a self) -> Result<Value, Error> {
		let tx = &self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();
		let iterator = tx.iterate(cf).await.unwrap();
		self.iterate(0, iterator)
	}

	pub async fn iterate_from_edge(
		&self,
		in_id: Uuid,
		t: &str,
		out_id: Uuid,
	) -> Result<Value, Error> {
		let tx = &self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();
		let t = &Identifier::new(t).unwrap();
		let key = self.edge_key(in_id, t, out_id);
		let iterator = tx.prefix_iterate(cf, key.to_vec()).await.unwrap();
		self.iterate(key.len(), iterator)
	}
}

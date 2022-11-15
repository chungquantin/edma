use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::util::{build_bytes, Component};
use crate::{Error, Identifier, SimpleTransaction};

impl_controller!(EdgePropertyController("edge-properties:v1"));

impl<'a> EdgePropertyController<'a> {
	fn key(&self, out_id: Uuid, t: &Identifier, in_id: Uuid) -> Vec<u8> {
		build_bytes(&[
			Component::Uuid(out_id),
			Component::Identifier(t),
			Component::Uuid(in_id),
			// Component::FixedLengthString(&name.0),
		])
		.unwrap()
	}

	pub async fn create(
		&self,
		out_id: Uuid,
		t: &Identifier,
		in_id: Uuid,
		data: Value,
	) -> Result<Value, Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		if !data.is_object() {
			panic!();
		}
		let o = data.as_object().unwrap();
		let cf = self.get_cf();

		for k in o.keys() {
			let val = o.get(k).unwrap();
			let json_value = build_bytes(&[Component::JsonValue(val)]).unwrap();
			let key = self.key(out_id, t, in_id);

			tx.set(cf.clone(), key, json_value).await.unwrap();
		}

		tx.commit().await.unwrap();
		Ok(data)
	}

	pub fn iterate(
		&self,
		iterator: Vec<Result<(Vec<u8>, Vec<u8>), Error>>,
	) -> Result<Value, Error> {
		let uuid_len = Component::Uuid(Uuid::nil()).len();
		let mut map: Map<String, Value> = Map::default();
		iterator.iter().for_each(|p| {
			let (k, v) = p.as_ref().unwrap();
			let attr = String::from_utf8(k[uuid_len..].to_vec()).unwrap();
			let value = Value::from(v.to_vec());
			map.insert(attr, value);
		});

		Ok(json!(map))
	}

	pub async fn iterate_for_all(&'a self) -> Result<Value, Error> {
		let tx = &self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();

		let iterator = tx.iterate(cf).await.unwrap();
		Ok(self.iterate(iterator).unwrap())
	}

	pub async fn iterate_from_attributes(
		&self,
		out_id: Uuid,
		t: &Identifier,
		in_id: Uuid,
	) -> Result<Value, Error> {
		let tx = &self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();

		let key = self.key(out_id, t, in_id);
		let iterator = tx.prefix_iterate(cf, key).await.unwrap();
		Ok(self.iterate(iterator).unwrap())
	}
}

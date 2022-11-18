use serde_json::Value;
use uuid::Uuid;

use crate::interface::{Key, KeyValuePair};
use crate::util::{build_bytes, from_i64_bytes, from_uuid_bytes, Component};
use crate::{Edge, Error, Identifier, SimpleTransaction};

impl_controller!(EdgeRepository("edges:v1"));

impl<'a> EdgeRepository<'a> {
	fn key(&self, in_id: Uuid, t: &Identifier, out_id: Uuid) -> Vec<u8> {
		build_bytes(&[Component::Uuid(in_id), Component::Uuid(out_id), Component::Identifier(t)])
			.unwrap()
	}

	pub async fn invert_e(&self, in_id: Uuid, t: &str, out_id: Uuid) -> Result<(Key, Edge), Error> {
		let t_id = &Identifier::new(t).unwrap();
		let key = self.key(out_id, t_id, in_id);
		let inverted_edge = Edge::new(in_id, t_id.clone(), out_id).unwrap();
		Ok((key, inverted_edge))
	}

	pub async fn add_e(
		&self,
		in_id: Uuid,
		t: &str,
		out_id: Uuid,
		bidirectional: bool,
	) -> Result<Edge, Error> {
		let cf = self.get_cf().unwrap();
		let mut tx = self.get_ds().transaction(true).unwrap();
		let t_id = Identifier::new(t).unwrap();
		let edge = Edge::new(in_id, t_id.clone(), out_id).unwrap();
		let timestamp_byte = edge.timestamp.to_be_bytes();
		let key = self.key(in_id, &t_id, out_id);
		tx.set(Some(cf.to_vec()), key, timestamp_byte).await.unwrap();
		if bidirectional {
			let (key, inverted_edge) = self.invert_e(in_id, t, out_id).await.unwrap();
			tx.set(Some(cf.to_vec()), key, timestamp_byte).await.unwrap();
			assert_eq!(edge, inverted_edge);
		}
		tx.commit().await.unwrap();
		Ok(edge)
	}

	pub async fn delete(
		&self,
		in_id: Uuid,
		t: &str,
		out_id: Uuid,
		bidirectional: bool,
	) -> Result<(), Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();
		let cf = self.get_cf().unwrap();
		let t = &Identifier::new(t).unwrap();
		let key = self.key(in_id, t, out_id);
		tx.del(Some(cf.to_vec()), key).await.unwrap();
		if bidirectional {
			// Deleting inverted edge
			let inverted_key = self.key(out_id, t, in_id);
			tx.del(Some(cf.to_vec()), inverted_key).await.unwrap();
		}
		tx.commit().await
	}

	pub async fn e(&self, in_id: Uuid, t: &str, out_id: Uuid) -> Result<Edge, Error> {
		let t = Identifier::new(t).unwrap();
		let edge = Edge::new(in_id, t, out_id).unwrap();
		Ok(edge)
	}

	async fn from_pair(&self, p: &KeyValuePair) -> Result<Edge, Error> {
		let (k, v) = p;
		let uuid_len = Component::Uuid(Uuid::nil()).len();
		let (in_id, out_id, t) = (&k[0..uuid_len], &k[uuid_len..uuid_len * 2], &k[uuid_len * 2..]);
		let t_id = Identifier::try_from(t.to_vec()).unwrap();
		let in_id = from_uuid_bytes(in_id).unwrap();
		let out_id = from_uuid_bytes(out_id).unwrap();
		let timestamp = from_i64_bytes(v.to_vec()).unwrap();
		let edge = Edge {
			in_id,
			out_id,
			t: t_id,
			timestamp,
			labels: Vec::default(),
			props: Value::default(),
		};

		Ok(edge)
	}

	pub async fn iterate_from_label(&self, t: &str) -> Result<Vec<Edge>, Error> {
		let tx = self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();

		let suffix = t.as_bytes();
		let iterator = tx.suffix_iterate(cf, suffix).await.unwrap();
		let mut result: Vec<Edge> = vec![];
		for p in iterator.iter() {
			let p_ref = p.as_ref().unwrap();
			let edge = self.from_pair(p_ref).await.unwrap();
			result.push(edge);
		}
		Ok(result)
	}

	pub async fn iterate_from_pair(&self, in_id: Uuid, out_id: Uuid) -> Result<Vec<Edge>, Error> {
		let tx = self.get_ds().transaction(false).unwrap();
		let cf = self.get_cf();

		let prefix = build_bytes(&[Component::Uuid(in_id), Component::Uuid(out_id)]).unwrap();
		let iterator = tx.prefix_iterate(cf, prefix).await.unwrap();
		let mut result: Vec<Edge> = vec![];
		for p in iterator.iter() {
			let p_ref = p.as_ref().unwrap();
			let edge = self.from_pair(p_ref).await.unwrap();
			result.push(edge);
		}
		Ok(result)
	}
}

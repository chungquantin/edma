use std::collections::HashMap;

use crate::interface::KeyValuePair;
use crate::storage::Transaction;
use crate::util::{
	build_byte_map, build_bytes, build_sized, build_usize_from_bytes, concat_bytes, Component,
};
use crate::{Error, SimpleTransaction};
use solomon_gremlin::structure::VertexPropertyMap;
use solomon_gremlin::{GValue, VertexProperty, GID};

impl_repository!(VertexPropertyRepository(VertexProperty));

fn build_vertex_property_value(value: &GValue) -> Vec<u8> {
	// let len = Component::GValue(value).len() + Component::GValueType(value).len();
	build_bytes(&[Component::GValueType(value), Component::GValue(value)]).unwrap()
}

fn build_vertex_property_key(vertex_id: &GID, id: &GID, label: &GValue) -> Vec<u8> {
	concat_bytes(vec![
		build_sized(Component::Gid(vertex_id)),
		build_sized(Component::GValue(label)),
		build_sized(Component::Gid(id)),
	])
}

impl<'a> VertexPropertyRepository<'a> {
	/// The property()-step is used to add properties to the elements of the graph (sideEffect).
	/// Unlike addV() and addE(), property() is a full sideEffect step in that it does not return
	/// the property it created, but the element that streamed into it. Moreover, if property()
	/// follows an addV() or addE(), then it is "folded" into the previous step to enable vertex
	/// and edge creation with all its properties in one creation operation.
	pub async fn property(
		&self,
		tx: &mut Transaction,
		vertex_id: &GID,
		id: &GID,
		label: &GValue,
		value: &GValue,
	) -> Result<VertexProperty, Error> {
		let cf = self.cf();
		let key = build_vertex_property_key(vertex_id, id, label);
		let val = self.append_value(tx, vertex_id, id, label, value).await.unwrap();
		tx.set(cf, key.to_vec(), val).await.unwrap();
		let label = label.get::<String>().unwrap();
		Ok(VertexProperty::new(vertex_id, label, value.clone()))
	}

	async fn append_value(
		&self,
		tx: &mut Transaction,
		vertex_id: &GID,
		id: &GID,
		label: &GValue,
		value: &GValue,
	) -> Result<Vec<u8>, Error> {
		let cf = self.cf();
		let val = build_vertex_property_value(value);
		let key = build_vertex_property_key(vertex_id, id, label);

		let get_current_val = tx.get(cf, key.to_vec()).await;
		match get_current_val {
			Ok(v) => {
				let existing_val = v.unwrap_or_default();
				Ok([existing_val, val].concat())
			}
			Err(_) => Ok(vec![]),
		}
	}

	/// Method to iterate the pairs of byte data
	fn iterate(
		&self,
		iterator: Vec<Result<KeyValuePair, Error>>,
	) -> Result<VertexPropertyMap, Error> {
		let mut map = HashMap::<String, Vec<VertexProperty>>::new();
		iterator.iter().for_each(|p| {
			let (k, v) = p.as_ref().unwrap();
			// Handle deserializing and rebuild vertex stream
			let bytemap = &build_byte_map(vec!["vertex_id", "label", "id"], k.to_vec());
			let label = String::from_utf8(bytemap.get("label").unwrap().to_vec()).unwrap();
			let gid = GID::Bytes(bytemap.get("id").unwrap().to_vec());
			// Handle deserializing GValue
			let variant = build_usize_from_bytes(v[..1].to_vec());
			let value = GValue::from_bytes(variant, v[1..].to_vec());
			let property = VertexProperty::new(gid, label.clone(), value);
			map.entry(label).or_default().push(property);
		});

		Ok(map)
	}

	/// Method to iterate the pairs of byte data with prefix as vertex id
	pub async fn iterate_from_vertex(
		&self,
		tx: &Transaction,
		vertex_id: &GID,
	) -> Result<VertexPropertyMap, Error> {
		let cf = self.cf();
		let prefix = concat_bytes(vec![build_sized(Component::Gid(vertex_id))]);
		let iterator = tx.prefix_iterate(cf, prefix).await.unwrap();
		self.iterate(iterator)
	}

	pub async fn iterate_from_label(
		&self,
		tx: &Transaction,
		vertex_id: &GID,
		label: &GValue,
	) -> Result<VertexPropertyMap, Error> {
		let cf = self.cf();
		let prefix = concat_bytes(vec![
			build_sized(Component::Gid(vertex_id)),
			build_sized(Component::GValue(label)),
		]);
		let iterator = tx.prefix_iterate(cf, prefix).await.unwrap();
		self.iterate(iterator)
	}
}

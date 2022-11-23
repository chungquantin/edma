use std::collections::HashMap;

use crate::interface::KeyValuePair;
use crate::storage::Transaction;
use crate::util::{
	build_byte_array, build_bytemap, build_bytes, build_sized, build_usize_from_bytes, Component,
};
use crate::{Error, SimpleTransaction};
use gremlin::{GValue, Property, GID};

impl_repository!(PropertyRepository(Property));

fn build_property_value(value: &GValue) -> Vec<u8> {
	build_bytes(&[Component::GValueType(value), Component::GValue(value)]).unwrap()
}

type PropertyMap = HashMap<String, Property>;

impl<'a> PropertyRepository<'a> {
	/// The property()-step is used to add properties to the elements of the graph (sideEffect).
	/// Unlike addV() and addE(), property() is a full sideEffect step in that it does not return
	/// the property it created, but the element that streamed into it. Moreover, if property()
	/// follows an addV() or addE(), then it is "folded" into the previous step to enable vertex
	/// and edge creation with all its properties in one creation operation.
	pub async fn property(
		&self,
		tx: &mut Transaction,
		vertex_id: &GID,
		label: &GValue,
		value: &GValue,
	) -> Result<Property, Error> {
		let cf = self.cf();
		let val = build_property_value(value);
		let key = build_byte_array(vec![
			build_sized(Component::GID(vertex_id)),
			build_sized(Component::GValue(value)),
		]);
		tx.set(cf, key.to_vec(), val).await.unwrap();
		let label = label.get::<String>().unwrap();
		Ok(Property::new(label, value.clone()))
	}

	/// Method to iterate the pairs of byte data
	fn iterate(&self, iterator: Vec<Result<KeyValuePair, Error>>) -> Result<PropertyMap, Error> {
		let mut map = HashMap::<String, Property>::new();
		iterator.iter().for_each(|p| {
			let (k, v) = p.as_ref().unwrap();
			// Handle deserializing and rebuild vertex stream
			let bytemap = &build_bytemap(vec!["vertex_id", "label"], k.to_vec());
			let label_bytes = bytemap.get("label").unwrap().to_vec();
			let label = String::from_utf8(label_bytes).unwrap();
			// Handle deserializing GValue
			let variant = build_usize_from_bytes(v[..1].to_vec());
			let value = GValue::from_bytes(variant, v[1..].to_vec());
			let property = Property::new(label.clone(), value);
			map.insert(label, property);
		});

		Ok(map)
	}

	/// Method to iterate the pairs of byte data with prefix as vertex id
	pub async fn iterate_from_vertex(&self, vertex_id: &GID) -> Result<PropertyMap, Error> {
		let tx = &self.tx();
		let cf = self.cf();
		let prefix = build_sized(Component::GID(vertex_id));
		let iterator = tx.prefix_iterate(cf, prefix).await.unwrap();
		self.iterate(iterator)
	}
}

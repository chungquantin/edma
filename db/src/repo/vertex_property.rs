use crate::storage::Transaction;
use crate::util::{build_byte_array, build_bytes, build_gid, build_gvalue, Component};
use crate::{Error, SimpleTransaction};
use gremlin::{GValue, Property, GID};

impl_controller!(VertexPropertyRepository("vertex-properties:v1"));

fn build_property_value(value: &GValue) -> Vec<u8> {
	build_bytes(&[Component::GValueType(&value), Component::GValue(&value)]).unwrap()
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
		label: &str,
		value: GValue,
	) -> Result<Property, Error> {
		let cf = self.cf();
		let val = build_property_value(&value);
		let key = build_byte_array(vec![build_gid(vertex_id), build_gvalue(&value)]);
		tx.set(cf, key.to_vec(), val).await.unwrap();
		Ok(Property::new(label, value))
	}

	// fn iterate(&self, iterator: Vec<Result<KeyValuePair, Error>>) -> Result<Value, Error> {
	// 	let uuid_len = Component::Uuid(Uuid::nil()).len();
	// 	let mut map: Map<String, Value> = Map::default();
	// 	iterator.iter().for_each(|p| {
	// 		let (k, v) = p.as_ref().unwrap();
	// 		let attr = String::from_utf8(k[uuid_len..].to_vec()).unwrap();
	// 		let value = build_json_value(v.to_vec()).unwrap();
	// 		map.insert(attr, value);
	// 	});

	// 	Ok(json!(map))
	// }

	pub async fn iterate_from_vertex(&self, vertex_id: &GID) -> Result<(), Error> {
		let tx = &self.tx();
		let cf = self.cf();
		let prefix = build_gid(vertex_id);
		let iterator = tx.prefix_iterate(cf, prefix).await.unwrap();
		// self.iterate(iterator);
		Ok(())
	}
}

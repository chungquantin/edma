use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
	util::{build_bytes, build_meta, deserialize_byte_data, Component},
	AccountDiscriminator, Error,
};

#[derive(PartialEq, Serialize, Deserialize, Eq, Debug, Clone)]
pub enum PropType {
	Unknown = 0,
	String = 1,
	UInt32 = 2,
	UInt64 = 3,
	UInt128 = 4,
	Document = 5,
	VecString = 6,
	VecUint32 = 7,
	VecUint64 = 8,
	VecUint128 = 9,
}

impl Default for PropType {
	fn default() -> Self {
		PropType::Unknown
	}
}

/// ## Property
/// Nodes and relationships can have properties (key-value pairs),
/// which further describe them.
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Property {
	pub id: Uuid,
	pub t: PropType,
	pub name: String,
}

impl Property {
	pub fn new(name: &str, t: PropType) -> Result<Self, Error> {
		Ok(Property {
			id: Uuid::new_v4(),
			name: name.to_string(),
			t,
		})
	}

	pub fn multi_serialize(props: &HashMap<Uuid, Vec<u8>>) -> Result<Vec<u8>, Error> {
		let prop_components =
			props.iter().map(|(id, val)| Component::Property(id, val)).collect::<Vec<Component>>();
		// Handle byte concatenate for property components
		let _prop_discriminator = AccountDiscriminator::Property.serialize();
		let _props = &build_bytes(&prop_components).unwrap();
		// (Property discriminator, Property byte array - meta for each property generated)
		let (p_dis, p) = (_prop_discriminator.as_slice(), _props.as_slice());
		let props_concat = [p_dis, p].concat();

		Ok(props_concat)
	}

	pub fn serialize(prop: &Property) -> Result<Vec<u8>, Error> {
		// First four bytes are the property
		let serialized_variant = bincode::serialize::<PropType>(&prop.t).unwrap();
		let property_meta = &build_meta(1, serialized_variant.len());
		let name = prop.name.as_bytes();
		let name_meta = &build_meta(1, name.len());
		// Dynamic length string will be concatenated at the end
		let val = [property_meta, &serialized_variant, name_meta, name].concat();
		Ok(val)
	}

	pub fn deserialize(v: Vec<u8>) -> Result<(String, PropType), Error> {
		let deserialized = deserialize_byte_data(v, false).unwrap();
		let property = &deserialized[0].0.first().unwrap();
		let name = &deserialized[1].0.first().unwrap();

		let name = String::from_utf8(name.to_vec()).unwrap();
		let t = bincode::deserialize::<PropType>(property).unwrap();

		Ok((name, t))
	}
}

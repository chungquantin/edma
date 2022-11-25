use super::Component;
use std::{
	collections::HashMap,
	io::{Cursor, Error as IoError},
};

/// Serializes component(s) into bytes.
pub fn build_bytes(components: &[Component]) -> Result<Vec<u8>, IoError> {
	let len = build_bytes_length(components).unwrap();
	let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

	for component in components {
		if let Err(err) = component.write(&mut cursor) {
			panic!("Could not write bytes: {}", err);
		}
	}

	Ok(cursor.into_inner())
}

pub fn build_bytes_length(components: &[Component]) -> Result<usize, IoError> {
	let len = components.iter().fold(0, |len, component| len + component.len());
	Ok(len)
}

pub fn concat_bytes(bytes_vec: Vec<Vec<u8>>) -> Vec<u8> {
	let mut components = vec![];
	for b in bytes_vec.iter() {
		components.push(Component::Bytes(b));
	}
	build_bytes(&components).unwrap()
}

pub fn build_sized(component: Component) -> Vec<u8> {
	let len = component.len();
	build_bytes(&[Component::Usize(len), component]).unwrap()
}

pub fn build_usize_from_bytes(bytes: Vec<u8>) -> usize {
	*bytes.first().unwrap() as usize
}

pub fn build_byte_map(key: Vec<&str>, bytes: Vec<u8>) -> HashMap<String, Vec<u8>> {
	let mut map = HashMap::<String, Vec<u8>>::new();
	let mut s = 0;
	let mut key_index = 0;
	while s < bytes.len() {
		let len = build_usize_from_bytes(bytes[s..s + 1].to_vec()) + 1;
		let data = bytes[s + 1..s + len].to_vec();
		map.insert(String::from(key[key_index]), data);

		s += len;
		key_index += 1;
	}
	map
}

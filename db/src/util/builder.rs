use gremlin::{GValue, LabelType, GID};

use super::Component;
use std::io::{Cursor, Error as IoError};

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

pub fn build_byte_array(bytes_vec: Vec<Vec<u8>>) -> Vec<u8> {
	let mut components = vec![];
	for b in bytes_vec.iter() {
		components.push(Component::Bytes(b));
	}
	build_bytes(&components).unwrap()
}

// TODO generic implement
pub fn build_gid(gid: &GID) -> Vec<u8> {
	let byte = Component::GID(gid);
	let len = byte.len();
	build_bytes(&[Component::Usize(len), byte]).unwrap()
}

pub fn build_gvalue(gvalue: &GValue) -> Vec<u8> {
	let byte = Component::GValue(gvalue);
	let len = byte.len();
	build_bytes(&[Component::Usize(len), byte]).unwrap()
}

pub fn build_label(label: &LabelType) -> Vec<u8> {
	let byte = Component::Label(label);
	let len = byte.len();
	build_bytes(&[Component::Usize(len), byte]).unwrap()
}

pub fn build_usize_from_bytes(bytes: Vec<u8>) -> usize {
	*bytes.first().unwrap() as usize
}

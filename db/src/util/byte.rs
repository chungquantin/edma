use std::io::{Cursor, Error as IoError, Write};

use rand::Rng;

pub enum Component<'a> {
	FixedLengthString(&'a str),
	Bytes(&'a [u8]),
	Usize(usize),
}

impl<'a> Component<'a> {
	pub fn len(&self) -> usize {
		match *self {
			Component::FixedLengthString(s) => s.len(),
			Component::Bytes(b) => b.len(),
			Component::Usize(_) => 1,
		}
	}

	pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
		match *self {
			Component::FixedLengthString(s) => cursor.write_all(s.as_bytes()),
			Component::Bytes(bytes) => cursor.write_all(bytes),
			Component::Usize(value) => cursor.write_all(&[value.try_into().unwrap()]),
		}
	}
}

pub fn generate_random_i32() -> i32 {
	let mut rng = rand::thread_rng();
	rng.gen::<i32>()
}

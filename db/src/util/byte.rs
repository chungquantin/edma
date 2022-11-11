use std::io::{Cursor, Error as IoError, Write};

use byteorder::{BigEndian, WriteBytesExt};
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};

use lazy_static::lazy_static;
use rand::Rng;
use uuid::Uuid;

use crate::{AccountDiscriminator, Identifier};

lazy_static! {
	/// The maximum possible datetime.
	pub static ref MAX_DATETIME: DateTime<Utc> =
		DateTime::from_utc(NaiveDateTime::from_timestamp(i64::from(i32::MAX), 0), Utc)
			.with_nanosecond(1_999_999_999u32)
			.unwrap();
}

pub enum Component<'a> {
	Uuid(Uuid),
	Property(&'a Uuid, &'a [u8]),
	FixedLengthString(&'a str),
	Identifier(&'a Identifier),
	DateTime(DateTime<Utc>),
	Bytes(&'a [u8]),
}

impl<'a> Component<'a> {
	pub fn len(&self) -> usize {
		match *self {
			Component::Uuid(_) => 16,
			Component::FixedLengthString(s) => s.len(),
			Component::Identifier(t) => t.0.len() + 1,
			Component::DateTime(_) => 8,
			Component::Bytes(b) => b.len(),
			Component::Property(_, d) => d.len(),
		}
	}

	pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
		match *self {
			Component::Uuid(uuid) => cursor.write_all(uuid.as_bytes()),
			Component::FixedLengthString(s) => cursor.write_all(s.as_bytes()),
			Component::Identifier(i) => {
				cursor.write_all(&[i.0.len() as u8])?;
				cursor.write_all(i.0.as_bytes())
			}
			Component::DateTime(datetime) => {
				let time_to_end = nanos_since_epoch(&MAX_DATETIME) - nanos_since_epoch(&datetime);
				cursor.write_u64::<BigEndian>(time_to_end)
			}
			Component::Bytes(bytes) => cursor.write_all(bytes),
			Component::Property(id, data) => {
				cursor.write_all(id.as_bytes())?; // Property id
				cursor.write_u8(data.len() as u8)?; // Length of property value
				cursor.write_all(data) // Property byte value
			}
		}
	}

	pub fn read_uuid(bytes: &[u8]) -> Result<Uuid, IoError> {
		let mut fix: [u8; 16] = Default::default();
		fix.copy_from_slice(&bytes[0..16]);
		Ok(Uuid::from_bytes(fix))
	}
}

/// Gets the number of nanoseconds since unix epoch for a given datetime.
///
/// # Arguments
/// * `datetime`: The datetime to convert.
fn nanos_since_epoch(datetime: &DateTime<Utc>) -> u64 {
	let timestamp = datetime.timestamp() as u64;
	let nanoseconds = u64::from(datetime.timestamp_subsec_nanos());
	timestamp * 1_000_000_000 + nanoseconds
}

// Serializes component(s) into bytes.
///
/// # Arguments
/// * `components`: The components to serialize to bytes.
pub fn build_bytes(components: &[Component]) -> Result<Vec<u8>, IoError> {
	let len = components.iter().fold(0, |len, component| len + component.len());
	let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

	for component in components {
		if let Err(err) = component.write(&mut cursor) {
			panic!("Could not write bytes: {}", err);
		}
	}

	Ok(cursor.into_inner())
}

pub fn from_uuid_bytes(bytes: &[u8]) -> Result<Uuid, IoError> {
	let l = Component::Uuid(Uuid::nil()).len();
	let slice = &bytes[0..l];
	let component = Component::read_uuid(slice).unwrap();
	Ok(component)
}

pub fn from_vec_uuid_bytes(bytes_vec: &Vec<u8>) -> Result<Vec<Uuid>, IoError> {
	let mut i = 0;
	let l = Component::Uuid(Uuid::nil()).len();
	let mut ans = vec![];
	if !bytes_vec.is_empty() {
		loop {
			let slice = &bytes_vec[i * l..(i + 1) * l];
			if slice.is_empty() {
				return Ok(ans);
			}
			let component = from_uuid_bytes(&slice.to_vec()).unwrap();
			ans.push(component);
			i += 1;
		}
	}
	Ok(ans)
}

pub fn from_vec_bytes(bytes_vec: &Vec<u8>) -> Result<Vec<Vec<u8>>, IoError> {
	let mut i = 0;
	let l = Component::Uuid(Uuid::nil()).len();
	let mut ans = vec![];
	if !bytes_vec.is_empty() {
		loop {
			let slice = &bytes_vec[i * l..(i + 1) * l];
			if slice.is_empty() {
				return Ok(ans);
			}
			ans.push(slice.to_vec());
			i += 1;
		}
	}
	Ok(ans)
}

pub fn generate_random_i32() -> i32 {
	let mut rng = rand::thread_rng();
	rng.gen::<i32>()
}

pub fn build_offset(size: u8, length: usize) -> Vec<u8> {
	vec![size, length as u8]
}

pub fn deserialize_data_with_offset(
	data: Vec<u8>,
	has_discriminator: bool,
) -> Result<(Vec<Vec<u8>>, usize), IoError> {
	let mut offset = 2;
	let mut discriminator = AccountDiscriminator::None;
	if has_discriminator {
		offset = 6;
		discriminator = bincode::deserialize(&data[..offset]).unwrap();
	}

	match discriminator {
		AccountDiscriminator::Property => todo!(),
		_ => {
			let (size, length) = (&data[offset - 2], &data[offset - 1]);
			println!("size: {} - length: {} - data: {:?}", size, length, data);
			let len = size * length;
			let d = data[offset..len as usize + offset].to_vec();

			let mut ans = Vec::new();

			for i in 0..*size {
				let ind = |x: u8| (x * length) as usize;
				let slice = &d[ind(i)..ind(i + 1)];
				ans.push(slice.to_vec());
			}
			Ok((ans, d.len() + offset))
		}
	}
}

type DataArray = Vec<Vec<u8>>;

pub fn deserialize_full_data(
	data: Vec<u8>,
	has_discriminator: bool,
) -> Result<Vec<DataArray>, IoError> {
	let mut result = vec![];
	let mut total_length = data.len();
	let mut start = 0;
	while total_length > 0 {
		let slice = data[start..].to_vec();
		let (data, length) = deserialize_data_with_offset(slice, has_discriminator).unwrap();

		result.push(data);
		start += length;
		total_length -= length;
	}

	Ok(result)
}

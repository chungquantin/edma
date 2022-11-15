use std::io::{Cursor, Error as IoError, Write};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};

use lazy_static::lazy_static;
use rand::Rng;
use serde_json::Value;
use uuid::Uuid;

use crate::{AccountDiscriminator, Error, Identifier};

type ByteData = Vec<u8>;
type ByteDataArray = Vec<ByteData>;
// Description: (List of return bytes, length of bytes vec, discriminator)
type DeserializeResult = Result<(ByteDataArray, usize, ByteData), IoError>;

lazy_static! {
	/// The maximum possible datetime.
	pub static ref MAX_DATETIME: DateTime<Utc> =
		DateTime::from_utc(NaiveDateTime::from_timestamp(i64::from(i32::MAX), 0), Utc)
			.with_nanosecond(1_999_999_999u32)
			.unwrap();
}

pub enum Component<'a> {
	Uuid(Uuid),
	FixedLengthString(&'a str),
	Identifier(&'a Identifier),
	DateTime(DateTime<Utc>),
	Bytes(&'a [u8]),
	JsonValue(&'a Value),
	JsonValueType(&'a Value),
}

impl<'a> Component<'a> {
	pub fn len(&self) -> usize {
		match *self {
			Component::Uuid(_) => 16,
			Component::FixedLengthString(s) => s.len(),
			Component::Identifier(t) => t.0.len() + 1,
			Component::DateTime(_) => 8,
			Component::Bytes(b) => b.len(),
			Component::JsonValue(v) | Component::JsonValueType(v) => v.to_string().len(),
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
			Component::JsonValueType(value) => match value {
				v if v.is_string() => cursor.write_all(&[1]),
				v if v.is_boolean() => cursor.write_all(&[2]),
				v if v.is_i64() => cursor.write_all(&[3]),
				v if v.is_u64() => cursor.write_all(&[4]),
				v if v.is_f64() => cursor.write_all(&[5]),
				_ => unimplemented!(),
			},
			Component::JsonValue(value) => match value {
				v if v.is_string() => cursor.write_all(v.as_str().unwrap().as_bytes()),
				v if v.is_boolean() => cursor
					.write_all(&[unsafe { std::mem::transmute::<bool, u8>(v.as_bool().unwrap()) }]),
				v if v.is_i64() => cursor.write_i64::<BigEndian>(v.as_i64().unwrap()),
				v if v.is_u64() => cursor.write_u64::<BigEndian>(v.as_u64().unwrap()),
				v if v.is_f64() => cursor.write_f64::<BigEndian>(v.as_f64().unwrap()),
				_ => unimplemented!(),
			},
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

pub fn from_uuid_bytes(bytes: &[u8]) -> Result<Uuid, IoError> {
	let l = Component::Uuid(Uuid::nil()).len();
	let slice = &bytes[0..l];
	let component = Component::read_uuid(slice).unwrap();
	Ok(component)
}

pub fn generate_random_i32() -> i32 {
	let mut rng = rand::thread_rng();
	rng.gen::<i32>()
}

pub fn build_meta(size: u8, length: usize) -> Vec<u8> {
	vec![size, length as u8]
}

/// # Deserialize data with metadata
/// Metadata: [size, length]
///
/// Based on the information of metadata to slice the raw byte data
pub fn deserialize_data_with_meta(data: ByteData, has_discriminator: bool) -> DeserializeResult {
	let meta_length = 2;
	let mut offset = meta_length;
	let mut discriminator = AccountDiscriminator::None;
	// If byte data includes discriminator, offset will be
	// - meta length (2) + discriminator length (4) = offset (6)
	if has_discriminator {
		let discriminator_length = 4;
		offset = meta_length + discriminator_length;
		// Binary deserialize byte data into AccountDiscriminator
		discriminator = bincode::deserialize(&data[..offset - 2]).unwrap();
	}

	let (size, length) = (&data[offset - 2], &data[offset - 1]);
	let len = size * length;
	let d = data[offset..len as usize + offset].to_vec();

	let mut ans = Vec::new();

	for i in 0..*size {
		let ind = |x: u8| (x * length) as usize;
		let slice = &d[ind(i)..ind(i + 1)];
		ans.push(slice.to_vec());
	}
	Ok((ans, d.len() + offset, discriminator.serialize()))
}

pub fn deserialize_byte_data(
	data: ByteData,
	has_discriminator: bool,
) -> Result<Vec<(ByteDataArray, ByteData)>, IoError> {
	let mut result = vec![];
	let mut total_length = data.len();
	let mut start = 0;
	while total_length > 0 {
		let slice = data[start..].to_vec();
		let (data, length, discriminator) =
			deserialize_data_with_meta(slice, has_discriminator).unwrap();

		result.push((data, discriminator));
		start += length;
		total_length -= length;
	}

	Ok(result)
}

pub fn build_json_value(v: Vec<u8>) -> Result<Value, Error> {
	let variant = &v[..1];
	let v = v[1..].to_vec();
	let value = match variant {
		// String
		[1] => Value::from(String::from_utf8(v).unwrap()),
		// Boolean
		[2] => Value::from(v[0] != 0),
		// i64
		[3] => Value::from(BigEndian::read_i64(&v)),
		// u64
		[4] => Value::from(BigEndian::read_u64(&v)),
		// f64
		[5] => Value::from(BigEndian::read_f64(&v)),
		_ => unimplemented!(),
	};

	Ok(value)
}

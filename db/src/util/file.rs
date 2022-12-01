use std::{env::temp_dir, path::Path};

use crate::Error;

use super::generate_random_i32;

pub fn path_to_string(path: &Path) -> Result<String, Error> {
	match path.to_str() {
		Some(p) => Ok(p.to_string()),
		None => Err(Error::Ds("The DB path is not valid UTF-8".to_string())),
	}
}

pub fn generate_path(name: &str, id: Option<i32>) -> String {
	match name {
		"rocksdb" => generate_rocksdb_path(id),
		"redb" => generate_redb_path(id),
		_ => unimplemented!(),
	}
}

/// Generate a path to store data for RocksDB
fn generate_rocksdb_path(id: Option<i32>) -> String {
	let random_id: i32 = generate_random_i32();
	let id = &id.unwrap_or(random_id).to_string();
	let path = if cfg!(target_os = "linux") {
		"/dev/shm/".into()
	} else {
		temp_dir()
	}
	.join(format!("solomon-rocksdb-{}", id));

	String::from("rocksdb:") + &path_to_string(&path).unwrap()
}

/// Generate a path to store data for RocksDB
fn generate_redb_path(id: Option<i32>) -> String {
	let random_id: i32 = generate_random_i32();
	let id = &id.unwrap_or(random_id).to_string();
	let path = if cfg!(target_os = "linux") {
		"/dev/shm/".into()
	} else {
		temp_dir()
	}
	.join(format!("solomon-redb-{}", id));

	String::from("redb:") + &path_to_string(&path).unwrap()
}

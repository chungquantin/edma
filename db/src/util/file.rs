use std::{env::temp_dir, path::Path};

use crate::Error;

use super::generate_random_i32;

pub fn path_to_string(path: &Path) -> Result<String, Error> {
	match path.to_str() {
		Some(p) => Ok(p.to_string()),
		None => Err(Error::Ds("The DB path is not valid UTF-8".to_string())),
	}
}

/// Generate a path to store data for RocksDB
pub fn generate_path(id: Option<i32>) -> String {
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

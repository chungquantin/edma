use path_absolutize::*;
use std::{
	env::{self, temp_dir},
	path::Path,
};

use crate::Error;

use super::generate_random_i32;

pub fn path_to_string(path: &Path) -> Result<String, Error> {
	match path.to_str() {
		Some(p) => Ok(p.to_string()),
		None => Err(Error::Ds("The DB path is not valid UTF-8".to_string())),
	}
}

pub fn get_absolute_path(path: &str) -> String {
	let p = Path::new(path);
	let cwd = env::current_dir().unwrap();

	p.absolutize_from(&cwd).unwrap().to_str().unwrap().to_string()
}

pub fn generate_path(name: &str, id: Option<i32>) -> String {
	macro_rules! impl_database_path {
					($($name: expr),*) => {
									match name {
										$(
											$name => database_path(name, id),
										)*
										_ => unimplemented!()
									}
					};
	}
	impl_database_path!("rocksdb", "redb", "sled")
}

pub fn database_path(name: &str, id: Option<i32>) -> String {
	let random_id: i32 = generate_random_i32();
	let id = &id.unwrap_or(random_id).to_string();
	let path = if cfg!(target_os = "linux") {
		"/dev/shm/".into()
	} else {
		temp_dir()
	}
	.join(format!("edma-{}-{}", name, id));

	String::from(format!("{}:", name)) + &path_to_string(&path).unwrap()
}

use path_absolutize::*;
use std::{env, path::Path};

pub fn get_absolute_path(path: &str) -> String {
	let p = Path::new(path);
	let cwd = env::current_dir().unwrap();

	p.absolutize_from(&cwd).unwrap().to_str().unwrap().to_string()
}

pub fn get_db_absolute_path(path: &str) -> (String, String) {
	let separator = ':';
	let mut split = path.split(separator);
	let name = split.next().unwrap();
	let chunk = split.next().unwrap();
	let path = get_absolute_path(chunk);
	(name.to_owned(), path)
}

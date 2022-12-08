use path_absolutize::*;
use std::{
	env,
	path::{Path, PathBuf},
};

pub fn get_absolute_path_buf(pathbuf: PathBuf) -> String {
	let cwd = env::current_dir().unwrap();
	pathbuf.absolutize_from(&cwd).unwrap().to_str().unwrap().to_string()
}

pub fn get_absolute_path(path: &str) -> String {
	let p = Path::new(path);
	let cwd = env::current_dir().unwrap();

	p.absolutize_from(&cwd).unwrap().to_str().unwrap().to_string()
}

pub fn sanitize(s: &str) -> String {
	let s = s.split('"').nth(1);
	s.unwrap().to_string()
}

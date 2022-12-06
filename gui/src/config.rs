#[derive(Clone)]
pub struct Config {
	pub paths: Vec<String>,
}

impl Config {
	pub fn new() -> Self {
		Config {
			paths: vec!["rocksdb:./temp".to_string(), "rocksdb:./temp/v2".to_string()],
		}
	}
}

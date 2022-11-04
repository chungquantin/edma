use std::path::Path;

use crate::Error;

pub fn path_to_string(path: &Path) -> Result<String, Error> {
	match path.to_str() {
		Some(p) => Ok(p.to_string()),
		None => Err(Error::Ds("The DB path is not valid UTF-8".to_string())),
	}
}

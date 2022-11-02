use std::fmt::Display;

#[derive(Debug)]
pub enum StorageErr<'a> {
	DriverErr(&'a str),
}

impl<'a> Display for StorageErr<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StorageErr::DriverErr(parse_int_error) => write!(f, "{}", parse_int_error),
		}
	}
}

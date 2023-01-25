use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Hash, PartialEq, Eq)]
pub enum ColumnFamily {
	TestSuite,
}

lazy_static! {
	pub static ref COLUMN_FAMILIES: HashMap<ColumnFamily, String> =
		HashMap::from([(ColumnFamily::TestSuite, "test_suite:v1".to_string()),]);
	pub static ref CF_NAMES: Vec<&'static String> = COLUMN_FAMILIES.values().collect();
}

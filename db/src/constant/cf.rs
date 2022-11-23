use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Hash, PartialEq, Eq)]
pub enum ColumnFamily {
	TestSuite,
	Edge,
	VertexProperty,
	Property,
	Vertex,
}

lazy_static! {
	pub static ref COLUMN_FAMILIES: HashMap<ColumnFamily, String> = HashMap::from([
		(ColumnFamily::TestSuite, "test_suite:v1".to_string()),
		(ColumnFamily::Edge, "vertices:v1".to_string()),
		(ColumnFamily::VertexProperty, "vertex-properties:v1".to_string()),
		(ColumnFamily::Property, "properties:v1".to_string()),
		(ColumnFamily::Vertex, "vertices:v1".to_string())
	]);
	pub static ref CF_NAMES: Vec<&'static String> = COLUMN_FAMILIES.values().collect();
}

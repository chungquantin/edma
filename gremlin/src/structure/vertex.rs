use crate::structure::VertexProperty;
use crate::structure::GID;
use std::collections::hash_map::{IntoIter, Iter};
use std::collections::HashMap;
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Vertex {
	id: GID,
	label: String,
	properties: HashMap<String, Vec<VertexProperty>>,
}

impl Vertex {
	pub(crate) fn new<T>(
		id: GID,
		label: T,
		properties: HashMap<String, Vec<VertexProperty>>,
	) -> Vertex
	where
		T: Into<String>,
	{
		Vertex {
			id,
			label: label.into(),
			properties,
		}
	}

	pub fn id(&self) -> &GID {
		&self.id
	}

	pub fn label(&self) -> &String {
		&self.label
	}

	pub fn iter(&self) -> Iter<String, Vec<VertexProperty>> {
		self.properties.iter()
	}

	pub fn property(&self, key: &str) -> Option<&VertexProperty> {
		self.properties.get(key).and_then(|v| v.get(0))
	}
}

impl IntoIterator for Vertex {
	type Item = (String, Vec<VertexProperty>);
	type IntoIter = IntoIter<String, Vec<VertexProperty>>;
	fn into_iter(self) -> Self::IntoIter {
		self.properties.into_iter()
	}
}

impl std::cmp::Eq for Vertex {}

impl std::hash::Hash for Vertex {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl PartialEq for Vertex {
	fn eq(&self, other: &Vertex) -> bool {
		&self.id == other.id()
	}
}

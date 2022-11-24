use uuid::Uuid;

use crate::structure::GID;
use crate::VertexProperty;
use std::collections::hash_map::{IntoIter, Iter};
use std::collections::HashMap;
use std::hash::Hasher;

pub type VertexPropertyMap = HashMap<String, Vec<VertexProperty>>;

#[derive(Debug, Clone)]
pub struct Vertex {
	id: GID,
	label: String,
	properties: VertexPropertyMap,
}

impl Default for Vertex {
	fn default() -> Self {
		let gid = GID::String(Uuid::new_v4().to_string());
		Vertex::partial_new(gid)
	}
}

/// ## Vertex
/// ### Description
/// A Vertex maintains pointers to both a set of incoming and outgoing Edge objects.
/// The outgoing edges are those edges for which the Vertex is the tail. The incoming edges
/// are those edges for which the Vertex is the head.
impl Vertex {
	pub fn new<T>(id: GID, label: T, properties: VertexPropertyMap) -> Vertex
	where
		T: Into<String>,
	{
		Vertex {
			id,
			label: label.into(),
			properties,
		}
	}

	pub fn partial_new(id: GID) -> Vertex {
		Vertex {
			id,
			label: Default::default(),
			properties: Default::default(),
		}
	}

	pub fn add_label<T>(&mut self, label: T)
	where
		T: Into<String>,
	{
		self.label = label.into();
	}

	pub fn add_property(&mut self, property: VertexProperty) {
		self.properties
			.entry(property.label().to_string())
			.or_insert(Vec::default())
			.push(property);
	}

	pub fn add_properties(&mut self, properties: VertexPropertyMap) {
		self.properties = properties;
	}

	pub fn id(&self) -> &GID {
		&self.id
	}

	pub fn label(&self) -> &String {
		&self.label
	}

	pub fn has_label(&self) -> bool {
		&self.label != ""
	}

	pub fn iter(&self) -> Iter<String, Vec<VertexProperty>> {
		self.properties.iter()
	}

	pub fn property(&self, key: &str) -> Option<&Vec<VertexProperty>> {
		self.properties.get(key)
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

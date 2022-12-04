use uuid::Uuid;

use crate::structure::{Property, Vertex, GID};
use std::collections::hash_map::{IntoIter, Iter};
use std::collections::HashMap;
use std::hash::Hasher;

pub type PropertyMap = HashMap<String, Property>;

#[derive(Debug, Clone)]
pub struct Edge {
	id: GID,
	label: String,
	in_v: Option<Vertex>,
	out_v: Option<Vertex>,
	properties: PropertyMap,
}

impl Default for Edge {
	fn default() -> Self {
		let gid = GID::String(Uuid::new_v4().to_string());
		Self {
			id: gid,
			label: Default::default(),
			in_v: Default::default(),
			out_v: Default::default(),
			properties: Default::default(),
		}
	}
}

impl Edge {
	pub fn new<T>(
		id: GID,
		label: T,
		in_v: Option<Vertex>,
		out_v: Option<Vertex>,
		properties: HashMap<String, Property>,
	) -> Edge
	where
		T: Into<String>,
	{
		Edge {
			id,
			label: label.into(),
			in_v,
			out_v,
			properties,
		}
	}

	pub fn partial_new(id: GID) -> Edge {
		Edge {
			id,
			label: Default::default(),
			properties: Default::default(),
			in_v: None,
			out_v: None,
		}
	}

	pub fn add_label<T>(&mut self, label: T)
	where
		T: Into<String>,
	{
		self.label = label.into();
	}

	pub fn add_property(&mut self, property: Property) {
		self.properties.entry(property.label().to_string()).or_insert(property);
	}

	pub fn add_properties(&mut self, properties: PropertyMap) {
		self.properties = properties;
	}

	pub fn id(&self) -> &GID {
		&self.id
	}

	pub fn label(&self) -> &String {
		&self.label
	}

	pub fn set_partial_in_v(&mut self, id: GID) {
		self.in_v = Some(Vertex::partial_new(id));
	}

	pub fn set_partial_out_v(&mut self, id: GID) {
		self.in_v = Some(Vertex::partial_new(id));
	}

	pub fn set_in_v(&mut self, v: Vertex) {
		self.in_v = Some(v);
	}

	pub fn set_out_v(&mut self, v: Vertex) {
		self.out_v = Some(v);
	}

	pub fn in_v(&self) -> &Option<Vertex> {
		&self.in_v
	}
	pub fn out_v(&self) -> &Option<Vertex> {
		&self.out_v
	}

	pub fn iter(&self) -> Iter<String, Property> {
		self.properties.iter()
	}

	pub fn property(&self, key: &str) -> Option<&Property> {
		self.properties.get(key)
	}
}

impl IntoIterator for Edge {
	type Item = (String, Property);
	type IntoIter = IntoIter<String, Property>;
	fn into_iter(self) -> Self::IntoIter {
		self.properties.into_iter()
	}
}

impl std::cmp::Eq for Edge {}

impl PartialEq for Edge {
	fn eq(&self, other: &Edge) -> bool {
		&self.id == other.id()
	}
}

impl std::hash::Hash for Edge {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

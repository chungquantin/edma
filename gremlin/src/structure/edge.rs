use crate::structure::{Property, Vertex, GID};
use std::collections::hash_map::{IntoIter, Iter};
use std::collections::HashMap;
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Edge {
	id: GID,
	label: String,
	in_v: Vertex,
	out_v: Vertex,
	properties: HashMap<String, Property>,
}

impl Edge {
	pub fn new<T>(
		id: GID,
		label: T,
		in_v_id: GID,
		in_v_label: T,
		out_v_id: GID,
		out_v_label: T,
		properties: HashMap<String, Property>,
	) -> Edge
	where
		T: Into<String>,
	{
		Edge {
			id,
			label: label.into(),
			in_v: Vertex::new(in_v_id, in_v_label, HashMap::new()),
			out_v: Vertex::new(out_v_id, out_v_label, HashMap::new()),
			properties,
		}
	}

	pub fn id(&self) -> &GID {
		&self.id
	}

	pub fn label(&self) -> &String {
		&self.label
	}

	pub fn in_v(&self) -> &Vertex {
		&self.in_v
	}
	pub fn out_v(&self) -> &Vertex {
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

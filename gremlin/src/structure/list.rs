use crate::GValue;

#[derive(Debug, PartialEq, Clone)]
pub struct List(Vec<GValue>);

impl Default for List {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl List {
	pub fn new(elements: Vec<GValue>) -> Self {
		List(elements)
	}

	pub(crate) fn take(self) -> Vec<GValue> {
		self.0
	}

	pub fn iter(&self) -> impl Iterator<Item = &GValue> {
		self.0.iter()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn push(&mut self, value: GValue) {
		self.0.push(value)
	}

	pub fn append(&mut self, other: &mut Vec<GValue>) {
		self.0.append(other)
	}

	pub fn core(self) -> Vec<GValue> {
		self.0
	}
}

impl std::iter::IntoIterator for List {
	type Item = GValue;
	type IntoIter = std::vec::IntoIter<Self::Item>;
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl Into<List> for Vec<GValue> {
	fn into(self) -> List {
		List(self)
	}
}

impl From<List> for Vec<GValue> {
	fn from(list: List) -> Self {
		list.take()
	}
}

impl std::ops::Index<usize> for List {
	type Output = GValue;

	fn index(&self, key: usize) -> &GValue {
		self.0.get(key).expect("no entry found for key")
	}
}

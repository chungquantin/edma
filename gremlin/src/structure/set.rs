use crate::GValue;
use std::vec::IntoIter;

// pub type Set = Vec<GValue>;

#[derive(Debug, PartialEq, Clone)]
pub struct Set(Vec<GValue>);

impl Set {
	pub(crate) fn take(self) -> Vec<GValue> {
		self.0
	}

	pub fn iter(&self) -> impl Iterator<Item = &GValue> {
		self.0.iter()
	}
}

impl Into<Set> for Vec<GValue> {
	fn into(self) -> Set {
		Set(self)
	}
}

impl From<Set> for Vec<GValue> {
	fn from(set: Set) -> Self {
		set.take()
	}
}

impl IntoIterator for Set {
	type Item = GValue;
	type IntoIter = IntoIter<GValue>;
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

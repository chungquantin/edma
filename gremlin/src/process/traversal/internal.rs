use crate::conversion::FromGValue;
use crate::process::traversal::GraphTraversal;

#[derive(Clone)]
pub struct MockTerminator {}

impl Default for MockTerminator {
	fn default() -> Self {
		MockTerminator {}
	}
}

impl MockTerminator {
	pub fn new() -> Self {
		MockTerminator {}
	}
}
impl<T: FromGValue> Terminator<T> for MockTerminator {
	type List = ();
	type Next = ();
	type HasNext = ();
	type Iter = ();

	fn to_list<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::List
	where
		E: Terminator<T>,
	{
		unimplemented!()
	}

	fn next<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::Next
	where
		E: Terminator<T>,
	{
		unimplemented!()
	}

	fn has_next<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
	where
		E: Terminator<T>,
	{
		unimplemented!()
	}

	fn iter<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::Iter
	where
		E: Terminator<T>,
	{
		unimplemented!()
	}
}
pub trait Terminator<T: FromGValue>: Clone {
	type List;
	type Next;
	type HasNext;
	type Iter;

	fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
	where
		E: Terminator<T>;

	fn next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Next
	where
		E: Terminator<T>;

	fn has_next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
	where
		E: Terminator<T>;

	fn iter<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Iter
	where
		E: Terminator<T>;
}

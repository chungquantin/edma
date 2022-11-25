use async_trait::async_trait;

use crate::conversion::FromGValue;
use crate::process::traversal::GraphTraversal;

#[async_trait]
pub trait Terminator<T: FromGValue>: Clone {
	type Executor;

	fn exec<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Executor
	where
		E: Terminator<T>;
}

#[derive(Clone)]
pub struct MockTerminator {}

impl Default for MockTerminator {
	fn default() -> Self {
		MockTerminator {}
	}
}

impl<T: FromGValue> Terminator<T> for MockTerminator {
	type Executor = T;

	fn exec<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::Executor
	where
		E: Terminator<T>,
	{
		todo!()
	}
}

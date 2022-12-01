use solomon_gremlin::{
	process::traversal::{GraphTraversal, Terminator},
	FromGValue,
};
use std::marker::Send;

use crate::storage::DatastoreRef;

use super::StepExecutor;

#[derive(Clone)]
pub struct GraphTerminator<'a> {
	datastore: DatastoreRef<'a>,
}

impl<'a> GraphTerminator<'a> {
	pub fn new(ds_ref: DatastoreRef<'a>) -> Self {
		GraphTerminator {
			datastore: ds_ref,
		}
	}
}

impl<'a, T: FromGValue + Send + Clone + 'static> Terminator<T> for GraphTerminator<'a> {
	type Executor = StepExecutor<'a, T>;

	fn exec<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Executor
	where
		E: Terminator<T>,
	{
		StepExecutor::<T>::new(traversal, self.datastore)
	}
}

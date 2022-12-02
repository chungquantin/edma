use crate::storage::DatastoreRef;
use solomon_gremlin::process::traversal::GraphTraversalSource;

use super::GraphTerminator;

type TraversalSource<'a> = GraphTraversalSource<GraphTerminator<'a>>;

pub struct Database<'a> {
	traversal: TraversalSource<'a>,
}

impl<'a> Database<'a> {
	pub fn new(ds_ref: DatastoreRef<'a>) -> Self {
		let terminator = GraphTerminator::new(ds_ref);
		let traversal = GraphTraversalSource::new(terminator);

		Database {
			traversal,
		}
	}

	pub fn traverse(&self) -> TraversalSource {
		self.traversal.clone()
	}
}

crate::full_database_test_impl!(redb_test, "redb");
crate::full_database_test_impl!(rocksdb_test, "rocksdb");

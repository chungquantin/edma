use std::sync::Arc;

mod tx;
mod ty;

use crate::{
	DBTransaction, DatastoreAdapter, Error, StorageAdapter, StorageAdapterName, StorageVariant,
};

use self::ty::{CassandraDBTransaction, DBType};

pub struct CassandraDBAdapter(StorageAdapter<String>);

impl CassandraDBAdapter {
	impl_new_type_adapter!(DBType);

	pub fn new(_path: &str) -> Result<CassandraDBAdapter, Error> {
		Ok(CassandraDBAdapter(StorageAdapter::<String>::new(
			StorageAdapterName::CassandraDB,
			String::from(""),
			StorageVariant::KeyValueStore,
		)?))
	}
}

impl DatastoreAdapter for CassandraDBAdapter {
	type Transaction = CassandraDBTransaction;

	fn default() -> Self {
		CassandraDBAdapter::new("").unwrap()
	}

	fn spawn(&self) -> Self {
		CassandraDBAdapter::default()
	}

	fn transaction(&self, rw: bool) -> Result<Self::Transaction, Error> {
		Ok(DBTransaction::<String, String>::new("".to_string(), Arc::pin("".to_string()), rw)
			.unwrap())
	}
}

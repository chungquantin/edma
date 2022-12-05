pub mod tx;
pub mod ty;

use redb::Database;
pub use tx::*;
pub use ty::*;

use crate::{
	util::generate_path, DBTransaction, DatastoreAdapter, Error, StorageAdapter,
	StorageAdapterName, StorageVariant,
};
pub struct ReDBAdapter(StorageAdapter<DBType>);

#[cfg(feature = "test-suite")]
crate::full_adapter_test_impl!(ReDBAdapter::default());

impl ReDBAdapter {
	impl_new_type_adapter!(DBType);

	pub fn new(path: &str) -> Result<ReDBAdapter, Error> {
		let path = &path["redb:".len()..];
		let db_instance = unsafe { Database::create(path).unwrap() };

		Ok(ReDBAdapter(StorageAdapter::<DBType>::new(
			StorageAdapterName::CassandraDB,
			path.to_string(),
			db_instance,
			StorageVariant::KeyValueStore,
		)?))
	}
}

impl DatastoreAdapter for ReDBAdapter {
	type Transaction = ReDBTransaction;

	fn default() -> Self {
		let path = &generate_path("redb", None);
		ReDBAdapter::new(path).unwrap()
	}

	fn spawn(&self) -> Self {
		ReDBAdapter::default()
	}

	fn path(&self) -> &str {
		&self.0.path
	}

	fn transaction(&self, w: bool) -> Result<Self::Transaction, Error> {
		let inner = self.get_initialized_inner().unwrap();
		let db = &inner.db_instance;
		let tx = db.begin_write().unwrap();

		let tx = unsafe { extend_tx_lifetime(tx) };

		Ok(DBTransaction::<DBType, TxType>::new(tx, db.clone(), w).unwrap())
	}
}

unsafe fn extend_tx_lifetime(tx: redb::WriteTransaction<'_>) -> redb::WriteTransaction<'static> {
	std::mem::transmute::<redb::WriteTransaction<'_>, redb::WriteTransaction<'static>>(tx)
}

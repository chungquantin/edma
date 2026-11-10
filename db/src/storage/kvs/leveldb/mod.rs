pub mod tx;
pub mod ty;

pub use tx::*;
pub use ty::*;

use std::marker::PhantomData;
use std::path::Path;

use async_trait::async_trait;
use leveldb::database::Database as LevelDBDatabase;
use leveldb::options::Options;

use crate::{
	util::{generate_path, get_absolute_path},
	DBTransaction, DatastoreAdapter, Error, StorageAdapter, StorageAdapterName, StorageVariant,
};
use ty::LevelDBKey;

pub struct LevelDBAdapter(StorageAdapter<DBType>);

#[cfg(feature = "test-suite")]
crate::full_adapter_test_impl!(LevelDBAdapter::default());

impl LevelDBAdapter {
	impl_new_type_adapter!(DBType);

	pub fn new(path: &str) -> Result<LevelDBAdapter, Error> {
		let path = &path["leveldb:".len()..];
		let abs_path = get_absolute_path(path);

		let mut opts = Options::new();
		opts.create_if_missing = true;

		let db_instance = LevelDBDatabase::<LevelDBKey>::open(Path::new(&abs_path), opts)
			.map_err(|e| Error::Ds(e.to_string()))?;

		Ok(LevelDBAdapter(StorageAdapter::<DBType>::new(
			StorageAdapterName::EchoDB,
			path.to_string(),
			db_instance,
			StorageVariant::KeyValueStore,
		)?))
	}
}

#[async_trait]
impl DatastoreAdapter for LevelDBAdapter {
	type Transaction = LevelDBTransaction;

	fn default() -> Self {
		let path = &generate_path("leveldb", None);
		LevelDBAdapter::new(path).unwrap()
	}

	fn spawn(&self) -> Self {
		LevelDBAdapter::default()
	}

	fn path(&self) -> &str {
		&self.0.path
	}

	async fn transaction(&self, rw: bool) -> Result<LevelDBTransaction, Error> {
		let inner = self.get_initialized_inner().unwrap();
		let db = &inner.db_instance;
		Ok(DBTransaction::<DBType, TxType>::new(PhantomData, db.clone(), rw).unwrap())
	}
}

use std::marker::PhantomData;

use async_trait::async_trait;
pub mod tx;
pub mod ty;

pub use tx::*;
pub use ty::*;

use crate::{
	util::{generate_path, get_absolute_path},
	DBTransaction, DatastoreAdapter, Error, StorageAdapter, StorageAdapterName, StorageVariant,
};
pub struct SledAdapter(StorageAdapter<DBType>);

#[cfg(feature = "test-suite")]
crate::full_adapter_test_impl!(SledAdapter::default());

impl SledAdapter {
	impl_new_type_adapter!(DBType);

	pub fn new(path: &str) -> Result<SledAdapter, Error> {
		let path = &path["redb:".len()..];
		let abs_path = get_absolute_path(path);
		let db_instance = sled::open(abs_path)?;

		Ok(SledAdapter(StorageAdapter::<DBType>::new(
			StorageAdapterName::ReDB,
			path.to_string(),
			db_instance,
			StorageVariant::KeyValueStore,
		)?))
	}
}

#[async_trait]
impl DatastoreAdapter for SledAdapter {
	type Transaction = SledTransaction;

	fn default() -> Self {
		let path = &generate_path("redb", None);
		SledAdapter::new(path).unwrap()
	}

	fn spawn(&self) -> Self {
		SledAdapter::default()
	}

	fn path(&self) -> &str {
		&self.0.path
	}

	async fn transaction(&self, w: bool) -> Result<Self::Transaction, Error> {
		let inner = self.get_initialized_inner().unwrap();
		let db = &inner.db_instance;
		Ok(DBTransaction::<DBType, TxType>::new(PhantomData, db.clone(), w).unwrap())
	}
}

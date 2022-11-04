use crate::{err::Error, tx::SimpleTransaction};
use async_trait::async_trait;
use std::{pin::Pin, sync::Arc};

pub enum StorageVariant {
	RelationalStore,
	KeyValueStore,
}

pub enum StorageAdapterName {
	RocksDB,
}

pub struct StorageAdapter<T> {
	pub name: StorageAdapterName,
	pub db_instance: Pin<Arc<T>>,
	pub variant: StorageVariant,
}

impl<T> StorageAdapter<T> {
	pub fn new(
		name: StorageAdapterName,
		db_instance: T,
		variant: StorageVariant,
	) -> Result<Self, Error> {
		Ok(StorageAdapter {
			name,
			db_instance: Arc::pin(db_instance),
			variant,
		})
	}
}

#[async_trait]
pub trait DatastoreAdapter<T>
where
	T: SimpleTransaction,
{
	// # Create new database transaction
	// Set `rw` default to false means readable but not readable
	fn transaction(&self, rw: bool) -> Result<T, Error>;

	fn default() -> Self;

	fn spawn(&self) -> Self;
}
